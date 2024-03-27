use super::{EventApplication, EventRepository};
use crate::{
    event::{error::Result, Event},
    id::Identifiable,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, event must
/// be retrived.
#[derive(Default)]
pub struct FindEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    id: <Event<EventRepo::Intv> as Identifiable>::Id,
}

impl<EventRepo> FindEvent<EventRepo>
where
    EventRepo: EventRepository,
{
    /// Executes the find query, through which one, and exactly one, event must
    /// be retrived.
    pub async fn execute(self) -> Result<Event<EventRepo::Intv>> {
        Ok(self.event_repo.find(self.id).await?.read().await.clone())
    }
}

impl<EventRepo> EventApplication<EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn find_event(
        &self,
        id: <Event<EventRepo::Intv> as Identifiable>::Id,
    ) -> FindEvent<EventRepo> {
        FindEvent {
            event_repo: self.event_repo.clone(),
            id,
        }
    }
}
