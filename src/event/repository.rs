use super::{service::EventRepository, Error, Event, Result};
use crate::{id::Id, interval::Interval};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<I>
where
    I: Interval,
{
    events: RwLock<HashMap<Id<Event<I>>, Event<I>>>,
}

impl<I> EventRepository for InMemoryEventRepository<I>
where
    I: Interval,
{
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
