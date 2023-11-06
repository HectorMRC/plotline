use super::{service::EventRepository, Error, Event, Result};
use crate::interval::{Interval, IntervalST};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<I>
where
    I: Interval,
{
    events: RwLock<IntervalST<Event<I>>>,
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

        if events.find(|e| e.id == event.id).is_some() {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.clone());
        Ok(())
    }
}
