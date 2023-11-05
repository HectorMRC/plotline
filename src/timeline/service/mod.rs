mod create;
pub use create::*;

use super::{Result, Timeline};
use crate::interval::Interval;
use std::sync::Arc;

pub trait TimelineRepository {
    type Interval: Interval;

    fn create(&self, timeline: &Timeline<Self::Interval>) -> Result<()>;
}

pub struct TimelineService<R> {
    pub timeline_repo: Arc<R>,
}
