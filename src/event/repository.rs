use serde::{Serialize, Deserialize};

use super::{service::EventRepository, Error, Event, Result};
use crate::{id::Id, interval::Interval, serde::{slice_from_hashmap, hashmap_from_slice}};
use std::{
    collections::HashMap,
    sync::RwLock,
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<I>
where I: Serialize + for<'a> Deserialize<'a> {
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    events: RwLock<HashMap<Id<Event<I>>, Event<I>>>,
}

impl<I> EventRepository for InMemoryEventRepository<I>
where I: Interval + Serialize + for<'a> Deserialize<'a> {
    type Interval = I;
    fn create(&self, event: &Event<Self::Interval>) -> Result<()> {
        let mut events = self
            .events
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if events.contains_key(&event.id) {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.id, event.clone());
        Ok(())
    }
}
