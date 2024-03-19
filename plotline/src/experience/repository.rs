use super::{
    application::{ExperienceFilter, ExperienceRepository},
    Error, Experience, Profile, Result,
};
use crate::{
    entity::{application::EntityRepository, repository::InMemoryEntityRepository, Entity},
    event::{application::EventRepository, repository::InMemoryEventRepository, Event},
    id::{Id, Identifiable},
    interval::Interval,
    macros::equals_or_return,
    resource::{Resource, ResourceMap, ResourceReadGuard, ResourceWriteGuard},
    serde::{from_rwlock, into_rwlock},
    transaction::{Tx, TxReadGuard, TxWriteGuard},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::RwLock,
};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RawProfile {
    entity: Id<Entity>,
    values: HashMap<String, String>,
}

impl Identifiable for RawProfile {
    type Id = Id<Entity>;

    fn id(&self) -> Self::Id {
        self.entity
    }
}

impl From<&Profile> for RawProfile {
    fn from(profile: &Profile) -> Self {
        RawProfile {
            entity: profile.entity.id(),
            values: profile.values.clone(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct RawExperience<Intv> {
    id: Id<Experience<Intv>>,
    entity: Id<Entity>,
    event: Id<Event<Intv>>,
    profiles: Vec<RawProfile>,
}

impl<Intv> Identifiable for RawExperience<Intv> {
    type Id = Id<Experience<Intv>>;

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl<Intv> From<&Experience<Intv>> for RawExperience<Intv> {
    fn from(experience: &Experience<Intv>) -> Self {
        RawExperience {
            id: experience.id(),
            entity: experience.entity.id(),
            event: experience.event.id(),
            profiles: experience.profiles.iter().map(Into::into).collect(),
        }
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(skip)]
    entity_repo: InMemoryEntityRepository,
    #[serde(skip)]
    event_repo: InMemoryEventRepository<Intv>,
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default
    )]
    experiences: RwLock<ResourceMap<RawExperience<Intv>>>,
}

impl<Intv> ExperienceRepository for InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Intv = Intv;
    type Tx = ExperienceAggregate<Intv>;

    fn find(&self, id: Id<Experience<Intv>>) -> Result<Self::Tx> {
        self.aggregate(
            self.experiences
                .read()
                .map_err(Error::from)?
                .get(&id)
                .cloned()
                .ok_or(Error::NotFound)?,
        )
    }

    fn filter(&self, filter: &ExperienceFilter<Intv>) -> Result<Vec<Self::Tx>> {
        self.experiences
            .read()
            .map_err(Error::from)?
            .values()
            .filter(|&entity| filter.matches(&entity.clone().read()))
            .cloned()
            .map(|experience| self.aggregate(experience))
            .collect()
    }

    fn create(&self, experience: &Experience<Intv>) -> Result<()> {
        let mut experiences = self.experiences.write().map_err(Error::from)?;

        if experiences.contains_key(&experience.id) {
            return Err(Error::AlreadyExists);
        }

        experiences.insert(experience.id, RawExperience::from(experience).into());
        Ok(())
    }

    fn delete(&self, id: Id<Experience<Intv>>) -> Result<()> {
        let mut experiences = self.experiences.write().map_err(Error::from)?;

        if experiences.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}

impl<Intv> InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    fn aggregate(
        &self,
        raw_experience: Resource<RawExperience<Intv>>,
    ) -> Result<ExperienceAggregate<Intv>> {
        let experience_tx = raw_experience.clone();
        let experience = experience_tx.read();

        let mut entities =
            HashSet::<Id<Entity>>::from_iter(experience.profiles.iter().map(Identifiable::id));

        entities.insert(experience.entity);

        // make sure (rw)locking is always done in the same order.
        let mut entities: Vec<_> = entities.into_iter().collect();
        entities.sort();

        Ok(ExperienceAggregate {
            experience: raw_experience,
            entities: entities
                .into_iter()
                .map(|id| self.entity_repo.find(id).map_err(Into::into))
                .collect::<Result<Vec<_>>>()?,
            event: self.event_repo.find(experience.event)?,
        })
    }
}

pub struct ExperienceAggregate<Intv> {
    experience: Resource<RawExperience<Intv>>,
    entities: Vec<Resource<Entity>>,
    event: Resource<Event<Intv>>,
}

impl<Intv> Tx<Experience<Intv>> for ExperienceAggregate<Intv>
where
    Intv: Interval,
{
    type ReadGuard<'a> = ExperienceAggregateReadGuard<'a, Intv> where Intv: 'a;
    type WriteGuard<'a> = ExperienceAggregateWriteGuard<'a, Intv> where Intv: 'a;

    fn read(&self) -> Self::ReadGuard<'_> {
        let experience = self.experience.read();
        let entities = self.entities.iter().map(Tx::read).collect::<Vec<_>>();
        let event = self.event.read();

        let data = Self::experience(&experience, &event, &entities);

        ExperienceAggregateReadGuard {
            _experience: experience,
            _entities: entities,
            _event: event,
            data,
        }
    }

    fn write(&self) -> Self::WriteGuard<'_> {
        let experience = self.experience.write();
        let entities = self.entities.iter().map(Tx::read).collect::<Vec<_>>();
        let event = self.event.read();

        let data = Self::experience(&experience, &event, &entities);

        ExperienceAggregateWriteGuard {
            experience,
            _entities: entities,
            _event: event,
            data,
        }
    }
}

impl<Intv> ExperienceAggregate<Intv>
where
    Intv: Interval,
{
    fn experience(
        experience: &RawExperience<Intv>,
        event: &Event<Intv>,
        entities: &[ResourceReadGuard<Entity>],
    ) -> Experience<Intv> {
        let find_or_default = |entity_id: Id<Entity>| -> Entity {
            entities
                .iter()
                .find(|&entity| entity.id() == entity_id)
                .map(Deref::deref)
                .cloned()
                .unwrap_or_else(|| Entity::default().with_id(entity_id))
        };

        Experience {
            id: experience.id(),
            entity: find_or_default(experience.entity),
            event: event.clone(),
            profiles: experience
                .profiles
                .iter()
                .map(|profile| Profile {
                    entity: find_or_default(profile.entity),
                    values: profile.values.clone(),
                })
                .collect(),
        }
    }
}

pub struct ExperienceAggregateReadGuard<'a, Intv> {
    _experience: ResourceReadGuard<'a, RawExperience<Intv>>,
    _entities: Vec<ResourceReadGuard<'a, Entity>>,
    _event: ResourceReadGuard<'a, Event<Intv>>,
    data: Experience<Intv>,
}

impl<'a, Intv> Deref for ExperienceAggregateReadGuard<'a, Intv> {
    type Target = Experience<Intv>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, Intv> TxReadGuard<Experience<Intv>> for ExperienceAggregateReadGuard<'a, Intv> {
    fn release(self) {}
}

pub struct ExperienceAggregateWriteGuard<'a, Intv> {
    experience: ResourceWriteGuard<'a, RawExperience<Intv>>,
    _entities: Vec<ResourceReadGuard<'a, Entity>>,
    _event: ResourceReadGuard<'a, Event<Intv>>,
    data: Experience<Intv>,
}

impl<'a, Intv> Deref for ExperienceAggregateWriteGuard<'a, Intv> {
    type Target = Experience<Intv>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, Intv> DerefMut for ExperienceAggregateWriteGuard<'a, Intv> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, Intv> TxWriteGuard<Experience<Intv>> for ExperienceAggregateWriteGuard<'a, Intv> {
    fn commit(mut self) {
        *self.experience = (&self.data).into()
    }

    fn rollback(self) {}
}

impl<Intv> ExperienceFilter<Intv> {
    fn matches(&self, experience: &RawExperience<Intv>) -> bool {
        equals_or_return!(self.id, &experience.id);
        equals_or_return!(self.entity, &experience.entity);
        equals_or_return!(self.event, &experience.event);
        true
    }
}
