use super::{EventRepository, EventService};
use crate::{event::Event, event::{Result, Error}, id::Id, name::Name, transaction::{Tx, TxGuard}};
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
    pub fn execute(self) -> Result<Event<EventRepo::Interval>> {
        match self.event_repo.find(self.id) {
            Ok(event_tx) => self.update(event_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err)
        }
    }

    fn create(self) -> Result<Event<EventRepo::Interval>>  {
        let (Some(name), Some(interval)) = (self.name, self.interval) else {
            return Err(Error::Custom("Name and interval must be set."))
        };

        let event = Event::new(self.id, name, interval);
        self.event_repo.create(&event)?;
        Ok(event)
    }

    fn update(self, event_tx: EventRepo::Tx) -> Result<Event<EventRepo::Interval>>  {
        let mut event = event_tx.begin()?;

        if let Some(name) = self.name {
            event.name = name;
        }
       
        if let Some(interval) = self.interval {
            event.interval = interval;
        }
        
        let data = event.clone();
        event.commit();
        
        Ok(data)
    }
}

impl<EventRepo, EntityRepo> EventService<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
{
    pub fn save_event(
        &self,
        id: Id<Event<EventRepo::Interval>>,
    ) -> SaveEvent<EventRepo> {
        SaveEvent {
            event_repo: self.event_repo.clone(),
            id,
            name: Default::default(),
            interval: Default::default()
        }
    }
}
