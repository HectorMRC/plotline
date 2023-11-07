mod create;
pub use create::*;

use super::{error::Result, Event};
use crate::{guard::Guard, id::Id, interval::Interval};
use std::sync::Arc;

pub trait EventRepository {
    type Interval: Interval;
    type Guard<'a>: Guard<'a, Event<Self::Interval>> where Self: 'a, Self::Interval: 'a;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
    fn find<'a>(&'a self, id: Id<Event<Self::Interval>>) -> Result<Self::Guard<'a>>;
}

pub struct EventService<R> {
    pub event_repo: Arc<R>,
}
