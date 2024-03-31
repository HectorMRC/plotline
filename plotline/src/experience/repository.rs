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
    resource::{
        from_rwlock, infallible_lock, into_rwlock, Resource, ResourceMap, ResourceReadGuard,
        ResourceWriteGuard,
    },
    transaction::{Tx, TxReadGuard, TxWriteGuard},
};
use futures::future;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
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
    Intv: Serialize + for<'a> Deserialize<'a>,
{
    #[serde(skip)]
    entity_repo: Arc<InMemoryEntityRepository>,
    #[serde(skip)]
    event_repo: Arc<InMemoryEventRepository<Intv>>,
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default
    )]
    experiences: RwLock<ResourceMap<RawExperience<Intv>>>,
}

impl<Intv> ExperienceRepository for InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a> + Sync + Send,
{
    type Intv = Intv;
    type Tx = ExperienceAggregate<Intv>;

    async fn find(&self, id: Id<Experience<Intv>>) -> Result<Self::Tx> {
        Ok(self.aggregate(
            infallible_lock(self.experiences.read())
                .get(&id)
                .cloned()
                .ok_or(Error::NotFound)?,
        ))
    }

    async fn filter(&self, filter: &ExperienceFilter<Intv>) -> Result<Vec<Self::Tx>> {
        let experiences: Vec<_> = infallible_lock(self.experiences.read())
            .values()
            .cloned()
            .collect();

        let mut matches = Vec::new();
        for experience_tx in experiences {
            let experience = experience_tx.read().await;
            if filter.matches(&experience) {
                matches.push(self.aggregate(experience_tx.clone()));
            }
        }

        Ok(matches)
    }

    async fn create(&self, experience: &Experience<Intv>) -> Result<()> {
        let mut experiences = infallible_lock(self.experiences.write());

        if experiences.contains_key(&experience.id) {
            return Err(Error::AlreadyExists);
        }

        experiences.insert(experience.id, RawExperience::from(experience).into());
        Ok(())
    }

    async fn delete(&self, id: Id<Experience<Intv>>) -> Result<()> {
        let mut experiences = infallible_lock(self.experiences.write());

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
    pub fn with_entity_repo(mut self, entity_repo: Arc<InMemoryEntityRepository>) -> Self {
        self.entity_repo = entity_repo;
        self
    }

    pub fn with_event_repo(mut self, event_repo: Arc<InMemoryEventRepository<Intv>>) -> Self {
        self.event_repo = event_repo;
        self
    }

    fn aggregate(
        &self,
        raw_experience: Resource<RawExperience<Intv>>,
    ) -> ExperienceAggregate<Intv> {
        ExperienceAggregate {
            experience: raw_experience,
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
        }
    }
}

pub struct ExperienceAggregate<Intv>
where
    Intv: Serialize + for<'a> Deserialize<'a>,
{
    experience: Resource<RawExperience<Intv>>,
    entity_repo: Arc<InMemoryEntityRepository>,
    event_repo: Arc<InMemoryEventRepository<Intv>>,
}

impl<Intv> Tx<Experience<Intv>> for ExperienceAggregate<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a> + Sync + Send,
{
    type ReadGuard<'a> = ExperienceAggregateReadGuard<'a, Intv> where Intv: 'a;
    type WriteGuard<'a> = ExperienceAggregateWriteGuard<'a, Intv> where Intv: 'a;

    async fn read(&self) -> Self::ReadGuard<'_> {
        let experience = self.experience.read().await;
        let (entities, event) =
            future::join(self.entities(&experience), self.event(&experience)).await;

        let data = Self::experience(&experience, &event, &entities);

        ExperienceAggregateReadGuard {
            _experience: experience,
            data,
        }
    }

    async fn write(&self) -> Self::WriteGuard<'_> {
        let experience = self.experience.write().await;
        let (entities, event) =
            future::join(self.entities(&experience), self.event(&experience)).await;

        let data = Self::experience(&experience, &event, &entities);

        ExperienceAggregateWriteGuard { experience, data }
    }
}

impl<Intv> ExperienceAggregate<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a> + Sync + Send,
{
    async fn entities(&self, experience: &RawExperience<Intv>) -> Vec<Entity> {
        let mut entities =
            HashSet::<Id<Entity>>::from_iter(experience.profiles.iter().map(Identifiable::id));

        entities.insert(experience.entity);

        // make sure (rw)locking is always done in the same order.
        let mut entities: Vec<_> = entities.into_iter().collect();
        entities.sort();

        future::join_all(entities.into_iter().map(|id| async move {
            if let Ok(entity_tx) = self.entity_repo.find(experience.entity).await {
                return entity_tx.read().await.clone();
            };

            Entity::default().with_id(id)
        }))
        .await
    }

    async fn event(&self, experience: &RawExperience<Intv>) -> Event<Intv> {
        if let Ok(event_tx) = self.event_repo.find(experience.event).await {
            return event_tx.read().await.clone();
        };

        Event::default().with_id(experience.event)
    }

    fn experience(
        experience: &RawExperience<Intv>,
        event: &Event<Intv>,
        entities: &[Entity],
    ) -> Experience<Intv> {
        let find_or_default = |entity_id: Id<Entity>| -> Entity {
            entities
                .iter()
                .find(|&entity| entity.id() == entity_id)
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
