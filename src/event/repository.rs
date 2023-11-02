use super::{service::EventRepository, Error, Event, EventId, Moment, Result};
use crate::{id::Id, interval::Interval};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

impl Interval for Arc<Event> {
    type Bound = Moment;

    fn lo(&self) -> Self::Bound {
        self.as_ref().lo()
    }

    fn hi(&self) -> Self::Bound {
        self.as_ref().hi()
    }
}

// #[derive(Default, Serialize, Deserialize)]
// #[serde(default)]
pub struct InMemoryEventRepository {
    events: RwLock<HashMap<Id<EventId>, Arc<Event>>>,
}

impl EventRepository for InMemoryEventRepository {
    fn create(&self, event: &Event) -> Result<()> {
        let mut events = self
            .events
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if events.contains_key(&event.id) {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.id, Arc::new(event.clone()));
        Ok(())
    }
}
