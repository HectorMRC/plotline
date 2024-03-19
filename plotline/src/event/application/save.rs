use super::{EventApplication, EventRepository};
use crate::{
    event::Event,
    event::{Error, Result},
    id::Id,
    name::{Error as NameError, Name},
    transaction::{Tx, TxWriteGuard},
    update_if_some,
};
use std::sync::Arc;

/// Implements the save event transaction.
pub struct SaveEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    id: Id<Event<EventRepo::Intv>>,
    name: Option<Name<Event<EventRepo::Intv>>>,
    interval: Option<EventRepo::Intv>,
}

impl<EventRepo> SaveEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn with_name(mut self, name: Option<Name<Event<EventRepo::Intv>>>) -> Self {
        self.name = name;
        self
    }

    pub fn with_interval(mut self, intv: Option<EventRepo::Intv>) -> Self {
        self.interval = intv;
        self
    }

    /// Executes the create event transaction.
    pub fn execute(self) -> Result<()> {
        match self.event_repo.find(self.id) {
            Ok(event_tx) => self.update(event_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let Some(name) = self.name else {
            return Err(NameError::NotAName.into());
        };

        let Some(interval) = self.interval else {
            return Err(Error::NotAnInterval);
        };

        let event = Event::new(self.id, name, interval);
        self.event_repo.create(&event)
    }

    fn update(self, event_tx: EventRepo::Tx) -> Result<()> {
        let mut event = event_tx.write();

        update_if_some(&mut event.name, self.name);
        update_if_some(&mut event.interval, self.interval);

        event.commit();
        Ok(())
    }
}

impl<EventRepo> EventApplication<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn save_event(&self, id: Id<Event<EventRepo::Intv>>) -> SaveEvent<EventRepo> {
        SaveEvent {
            event_repo: self.event_repo.clone(),
            id,
            name: Default::default(),
            interval: Default::default(),
        }
    }
}
