use super::{
    application::{ExperienceFilter, ExperienceRepository},
    Experience,
};
use crate::{
    entity::Entity,
    event::Event,
    id::Id,
    interval::Interval,
    serde::{hashmap_from_slice, slice_from_hashmap},
    transaction::Resource,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

type Repository<Intv> = RwLock<HashMap<(Id<Entity>, Id<Event<Intv>>), Resource<Experience<Intv>>>>;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryExperienceRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    events: Repository<Event<Intv>>,

    #[serde(skip)]
    entity_by_event: HashMap<Id<Event<Intv>>, Vec<Id<Entity>>>,

    #[serde(skip)]
    event_by_entity: HashMap<Vec<Id<Entity>>, Id<Event<Intv>>>,
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
