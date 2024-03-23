//! The application service.

mod save;
pub use save::*;

use super::{error::Result, Event};
use crate::{id::Id, interval::Interval, transaction::Tx};
use std::sync::Arc;

pub trait EventRepository {
    type Intv: Interval;
    type Tx: Tx<Event<Self::Intv>>;

    async fn create(&self, event: &Event<Self::Intv>) -> Result<()>;
    async fn find(&self, id: Id<Event<Self::Intv>>) -> Result<Self::Tx>;
}

pub struct EventApplication<EventRepo> {
    pub event_repo: Arc<EventRepo>,
}
