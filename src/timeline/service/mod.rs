mod create;
pub use create::*;

use crate::id::Id;

use super::{Result, Timeline};
use std::sync::Arc;

pub trait TimelineRepository {
    fn find(&self, id: &Id<Timeline>) -> Result<Timeline>;
    fn create(&self, timeline: &Timeline) -> Result<()>;
    fn save(&self, timeline: &Timeline) -> Result<()>;
}

pub struct TimelineService<R> {
    pub timeline_repo: Arc<R>,
}
