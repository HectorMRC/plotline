mod create;
pub use create::*;

use super::{error::Result, Event};
use std::sync::Arc;

pub trait EventRepository {
    fn create(&self, event: &Event) -> Result<()>;
}

pub struct EventService<R> {
    pub event_repo: Arc<R>,
}
