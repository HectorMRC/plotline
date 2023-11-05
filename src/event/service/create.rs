use super::{EventRepository, EventService};
use crate::{event::Event, event::Result, id::Id, name::Name};
use std::sync::Arc;

pub struct CreateEvent<R>
where
    R: EventRepository,
{
    event_repo: Arc<R>,
    name: Name<Event<R::Interval>>,
    interval: R::Interval,
    id: Option<Id<Event<R::Interval>>>,
}

impl<R> CreateEvent<R>
where
    R: EventRepository,
{
    /// Sets the optional id value.
    pub fn with_id(mut self, id: Option<Id<Event<R::Interval>>>) -> Self {
        self.id = id;
        self
    }

    /// Executes the create event transaction.
    pub fn execute(self) -> Result<Event<R::Interval>> {
        let event = if let Some(event_id) = self.id {
            Event::with_id(event_id, self.name, self.interval)
        } else {
            Event::new(self.name, self.interval)
        };

        self.event_repo.create(&event)?;
        Ok(event)
    }
}

impl<R> EventService<R>
where
    R: EventRepository,
{
    pub fn create_event(
        &self,
        name: Name<Event<R::Interval>>,
        interval: R::Interval,
    ) -> CreateEvent<R> {
        CreateEvent {
            event_repo: self.event_repo.clone(),
            name,
            interval,
            id: Default::default(),
        }
    }
}
