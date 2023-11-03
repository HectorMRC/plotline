use super::{TimelineRepository, TimelineService};
use crate::{
    id::Id,
    name::Name,
    timeline::{Result, Timeline},
};
use std::sync::Arc;

/// Implements the create timeline transaction,
pub struct CreateTimeline<R> {
    timeline_repo: Arc<R>,
    name: Name<Timeline>,
    id: Option<Id<Timeline>>,
}

impl<R> CreateTimeline<R>
where
    R: TimelineRepository,
{
    /// Executes the create timeline transaction.
    pub fn execute(self) -> Result<Timeline> {
        let mut timeline = if let Some(timeline_id) = self.id {
            Timeline::with_id(timeline_id, self.name)
        } else {
            Timeline::new(self.name)
        };

        self.timeline_repo.create(&timeline)?;
        Ok(timeline)
    }
}

impl<R> CreateTimeline<R> {
    pub fn with_id(mut self, id: Option<Id<Timeline>>) -> Self {
        self.id = id;
        self
    }
}

impl<R> TimelineService<R>
where
    R: TimelineRepository,
{
    pub fn create_timeline(&self, name: Name<Timeline>) -> CreateTimeline<R> {
        CreateTimeline {
            timeline_repo: self.timeline_repo.clone(),
            name,
            id: Default::default(),
        }
    }
}
