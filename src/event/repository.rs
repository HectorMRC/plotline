use super::{application::EventRepository, Error, Event, Result};
use crate::{
    id::Id,
    interval::Interval,
    serde::{hashmap_from_slice, slice_from_hashmap},
    transaction::Resource,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

type Repository<Intv> = RwLock<HashMap<Id<Event<Intv>>, Resource<Event<Intv>>>>;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    events: Repository<Intv>,
}

impl<Intv> EventRepository for InMemoryEventRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Interval = Intv;
    type Tx = Resource<Event<Intv>>;

    fn create(&self, event: &Event<Intv>) -> Result<()> {
        let mut events = self
            .events
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if events.contains_key(&event.id) {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.id, event.clone().into());
        Ok(())
    }

    fn find(&self, id: Id<Event<Intv>>) -> Result<Self::Tx> {
        let events = self
            .events
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?;

        events
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
            .map(Resource::from)
    }
}
