use futures::future;

use super::{EventApplication, EventRepository};
use crate::{
    event::{Event, Result},
    id::Id,
    name::Name,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the filter query, through which zero o more events may be
/// retrived.
#[derive(Default)]
pub struct EventFilter<Intv> {
    pub name: Option<Name<Event<Intv>>>,
    pub id: Option<Id<Event<Intv>>>,
}

impl<Intv> EventFilter<Intv> {
    pub fn with_id(mut self, id: Option<Id<Event<Intv>>>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: Option<Name<Event<Intv>>>) -> Self {
        self.name = name;
        self
    }
}

#[derive(Default)]
pub struct FilterEvents<EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    filter: EventFilter<EventRepo::Intv>,
}

impl<EventRepo> FilterEvents<EventRepo>
where
    EventRepo: EventRepository,
{
    /// Executes the filter query, through which zero o more events may be
    /// retrived.
    pub async fn execute(self) -> Result<Vec<Event<EventRepo::Intv>>> {
        Ok(future::join_all(
            self.event_repo
                .filter(&self.filter)
                .await?
                .into_iter()
                .map(|event_tx| async move { event_tx.read().await.clone() }),
        )
        .await)
    }
}

impl<EventRepo> FilterEvents<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn with_filter(mut self, filter: EventFilter<EventRepo::Intv>) -> Self {
        self.filter = filter;
        self
    }
}

impl<EventRepo> EventApplication<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn filter_events(&self) -> FilterEvents<EventRepo> {
        FilterEvents {
            event_repo: self.event_repo.clone(),
            filter: Default::default(),
        }
    }
}
