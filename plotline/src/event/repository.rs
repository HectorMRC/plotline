use super::{
    application::{EventFilter, EventRepository},
    Error, Event, Result,
};
use crate::{
    id::Id,
    interval::Interval,
    macros::equals_or_return,
    resource::{from_rwlock, infallible_lock, into_rwlock, Resource, ResourceMap},
    transaction::Tx,
};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEventRepository<Intv>
where
    Intv: Serialize + for<'a> Deserialize<'a>,
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
    Intv: Interval + Serialize + for<'a> Deserialize<'a> + Sync + Send,
{
    type Intv = Intv;
    type Tx = Resource<Event<Intv>>;

    async fn find(&self, id: Id<Event<Intv>>) -> Result<Self::Tx> {
        infallible_lock(self.events.read())
            .get(&id)
            .cloned()
            .ok_or(Error::NotFound)
    }

    async fn filter(&self, filter: &EventFilter<Self::Intv>) -> Result<Vec<Self::Tx>> {
        let events: Vec<_> = infallible_lock(self.events.read())
            .values()
            .cloned()
            .collect();

        let mut matches = Vec::new();
        for event_tx in events {
            let event = event_tx.read().await;
            if filter.matches(&event) {
                matches.push(event_tx.clone());
            }
        }

        Ok(matches)
    }

    async fn create(&self, event: &Event<Intv>) -> Result<()> {
        let mut events = infallible_lock(self.events.write());

        if events.contains_key(&event.id) {
            return Err(Error::AlreadyExists);
        }

        events.insert(event.id, event.clone().into());
        Ok(())
    }
}

impl<Intv> EventFilter<Intv> {
    fn matches(&self, event: &Event<Intv>) -> bool {
        equals_or_return!(self.name, &event.name);
        equals_or_return!(self.id, &event.id);
        true
    }
}
