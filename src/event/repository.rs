use super::{service::EventRepository, Error, Event, Result};
use crate::{
    transaction::Resource,
    id::Id,
    interval::Interval,
    serde::{hashmap_from_slice, slice_from_hashmap},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<I>
where
    I: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    events: RwLock<HashMap<Id<Event<I>>, Arc<Mutex<Event<I>>>>>,
}

impl<I> EventRepository for InMemoryEventRepository<I>
where
    I: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Interval = I;
    type Tx = Resource<Event<I>>;

    fn create(&self, event: &Event<I>) -> Result<()> {
        let mut events = self
            .events
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if events.contains_key(&event.id) {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.id, Arc::new(Mutex::new(event.clone())));
        Ok(())
    }

    fn find(&self, id: Id<Event<I>>) -> Result<Self::Tx> {
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
