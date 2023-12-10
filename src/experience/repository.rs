use super::{
    application::{ExperienceFilter, ExperienceRepository},
    Experience,
};
use crate::{
    entity::Entity,
    event::Event,
    id::Id,
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

    fn create(&self, _experience: &Experience<Intv>) -> super::Result<()> {
        todo!()
    }

    fn filter(&self, _filter: ExperienceFilter<Intv>) -> super::Result<Vec<Self::Tx>> {
        todo!()
    }
}
