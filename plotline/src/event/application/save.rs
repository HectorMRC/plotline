use super::{EventApplication, EventRepository};
use crate::{
    assign_inner_value,
    event::Event,
    event::{Error, Result},
    id::Id,
    name::{Error as NameError, Name},
    transaction::{Tx, TxGuard},
};
use std::sync::Arc;

/// Implements the save event transaction.
pub struct SaveEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    id: Id<Event<EventRepo::Interval>>,
    name: Option<Name<Event<EventRepo::Interval>>>,
    interval: Option<EventRepo::Interval>,
}

impl<EventRepo> SaveEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn with_name(mut self, name: Option<Name<Event<EventRepo::Interval>>>) -> Self {
        self.name = name;
        self
    }

    pub fn with_interval(mut self, intv: Option<EventRepo::Interval>) -> Self {
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
        let mut event = event_tx.begin();

        assign_inner_value(self.name, &mut event.name);
        assign_inner_value(self.interval, &mut event.interval);

        event.commit();
        Ok(())
    }
}

impl<EventRepo> EventApplication<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn save_event(&self, id: Id<Event<EventRepo::Interval>>) -> SaveEvent<EventRepo> {
        SaveEvent {
            event_repo: self.event_repo.clone(),
            id,
            name: Default::default(),
            interval: Default::default(),
        }
    }
}
