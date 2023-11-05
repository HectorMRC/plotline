mod create;
pub use create::*;

use super::{error::Result, Event};
use crate::interval::Interval;
use std::sync::Arc;

pub trait EventRepository {
    type Interval: Interval;

    fn create(&self, event: &Event<Self::Interval>) -> Result<()>;
}

pub struct EventService<R> {
    pub event_repo: Arc<R>,
}
