use std::sync::RwLock;

use super::{application::EventRepository, Error, Event, Result};
use crate::{
    id::Id,
    interval::Interval,
    resource::{Resource, ResourceMap},
    serde::{from_rwlock, into_rwlock},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default
    )]
    events: RwLock<ResourceMap<Event<Intv>>>,
}

impl<Intv> EventRepository for InMemoryEventRepository<Intv>
where
    Intv: Interval + Serialize + for<'a> Deserialize<'a>,
{
    type Intv = Intv;
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

        events.get(&id).cloned().ok_or(Error::NotFound)
    }
}
