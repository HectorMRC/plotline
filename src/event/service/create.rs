use super::{EventRepository, EventService};
use crate::{event::Event, event::Result, id::Id, name::Name};
use std::sync::Arc;

/// Implements the create event transaction.
pub struct CreateEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    name: Name<Event<EventRepo::Interval>>,
    interval: EventRepo::Interval,
    id: Option<Id<Event<EventRepo::Interval>>>,
}

impl<EventRepo> CreateEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    /// Sets the optional id value.
    pub fn with_id(mut self, id: Option<Id<Event<EventRepo::Interval>>>) -> Self {
        self.id = id;
        self
    }

    /// Executes the create event transaction.
    pub fn execute(self) -> Result<Event<EventRepo::Interval>> {
        let event = if let Some(event_id) = self.id {
            Event::with_id(event_id, self.name, self.interval)
        } else {
            Event::new(self.name, self.interval)
        };

        self.event_repo.create(&event)?;
        Ok(event)
    }
}

impl<EventRepo, EntityRepo> EventService<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
{
    pub fn create_event(
        &self,
        name: Name<Event<EventRepo::Interval>>,
        interval: EventRepo::Interval,
    ) -> CreateEvent<EventRepo> {
        CreateEvent {
            event_repo: self.event_repo.clone(),
            name,
            interval,
            id: Default::default(),
        }
    }
}
