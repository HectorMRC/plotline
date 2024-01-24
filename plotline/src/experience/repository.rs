use super::{
    application::{ExperienceFilter, ExperienceRepository},
    Error, Experience,
};
use crate::{
    entity::Entity,
    event::Event,
    id::{Id, Identifiable},
    interval::Interval,
    resource::{Resource, ResourceMap},
    serde::{from_rwlock, into_rwlock},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
struct Repository<Intv> {
    #[serde(default)]
    experiences: ResourceMap<Experience<Intv>>,

    #[serde(skip)]
    entity_by_event: HashMap<Id<Event<Intv>>, HashSet<Id<Entity>>>,

    #[serde(skip)]
    event_by_entity: HashMap<Id<Entity>, HashSet<Id<Event<Intv>>>>,
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default,
        flatten
    )]
    data: RwLock<Repository<Intv>>,
}

impl<Intv> ExperienceRepository for InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Interval = Intv;
    type Tx = Resource<Experience<Intv>>;

    fn create(&self, experience: &Experience<Intv>) -> super::Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|err| err.to_string())
            .map_err(Error::Lock)?;

        let experience_id @ (entity_id, event_id) = experience.id();
        if data.experiences.contains_key(&experience_id) {
            return Err(Error::AlreadyExists);
        }

        data.experiences
            .insert(experience_id, experience.clone().into());

        if let Some(events) = data.event_by_entity.get_mut(&entity_id) {
            events.insert(event_id);
        } else {
            data.event_by_entity
                .insert(entity_id, HashSet::from_iter([event_id]));
        }

        if let Some(entities) = data.entity_by_event.get_mut(&event_id) {
            entities.insert(entity_id);
        } else {
            data.entity_by_event
                .insert(event_id, HashSet::from_iter([entity_id]));
        }

        Ok(())
    }

    fn filter(&self, filter: &ExperienceFilter<Intv>) -> super::Result<Vec<Self::Tx>> {
        let data = self
            .data
            .read()
            .map_err(|err| err.to_string())
            .map_err(Error::Lock)?;

        if let (Some(entity_id), Some(event_id)) = (filter.entity, filter.event) {
            return Ok(data
                .experiences
                .get(&(entity_id, event_id))
                .cloned()
                .map(|experience| vec![experience])
                .unwrap_or_default());
        };

        if let Some(entity_id) = filter.entity {
            return Ok(data
                .event_by_entity
                .get(&entity_id)
                .map(|events| {
                    events
                        .iter()
                        .filter_map(|event_id| {
                            data.experiences.get(&(entity_id, *event_id)).cloned()
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default());
        }

        if let Some(event_id) = filter.event {
            return Ok(data
                .entity_by_event
                .get(&event_id)
                .map(|entities| {
                    entities
                        .iter()
                        .filter_map(|entity_id| {
                            data.experiences.get(&(*entity_id, event_id)).cloned()
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default());
        }

        Ok(Vec::new())
    }
}
