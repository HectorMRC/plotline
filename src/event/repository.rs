use super::{service::EventRepository, Error, Event, Result};
use crate::{id::Id, interval::Interval, guard::Guarded};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::{Arc, RwLock, Mutex}};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<I>
where
    I: Interval,
{
    events: RwLock<HashMap<Id<Event<I>>, Arc<Mutex<Event<I>>>>>,
}

impl<I> EventRepository for InMemoryEventRepository<I>
where
    I: Interval,
{
    type Interval = I;
    type Guard<'a> = Guarded<'a, Event<Self::Interval>> where Self: 'a,  Self::Interval: 'a;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()> {
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

    fn find<'a>(&'a self, id: Id<Event<Self::Interval>>) -> Result<Self::Guard<'a>> {
        let events = self
            .events
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?;

        let Some(event) = events.get(&id).cloned() else {
            return Err(Error::NotFound);
        };

        event.try_into().map_err(|_| Error::NotFound)
    }

    
}
