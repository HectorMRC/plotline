mod create;
pub use create::*;

use super::{error::Result, Event};
use crate::{guard::Guard, id::Id, interval::Interval};
use std::sync::Arc;

/// An EventGuard holds an [Event], ensuring its atomicity.
pub trait EventGuard<I>: Guard<Event<I>> {}

pub trait EventRepository {
    type Interval: Interval;
    type Guard: EventGuard<Self::Interval>;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
    fn find(&self, id: Id<Event<Self::Interval>>) -> Result<Self::Guard>;
}

pub struct EventService<R> {
    pub event_repo: Arc<R>,
}
