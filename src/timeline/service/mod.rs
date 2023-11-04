mod create;
pub use create::*;

use crate::id::Id;

use super::{Result, Timeline};
use std::sync::Arc;

pub trait TimelineRepository {
    fn create(&self, timeline: &Timeline) -> Result<()>;
}

pub struct TimelineService<R> {
    pub timeline_repo: Arc<R>,
}
