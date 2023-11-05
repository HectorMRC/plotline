use super::{TimelineRepository, TimelineService};
use crate::{
    id::Id,
    name::Name,
    timeline::{Result, Timeline},
};
use std::sync::Arc;

/// Implements the create timeline transaction,
pub struct CreateTimeline<R> 
where
    R: TimelineRepository {
    timeline_repo: Arc<R>,
    name: Name<Timeline<R::Interval>>,
    id: Option<Id<Timeline<R::Interval>>>,
}

impl<R> CreateTimeline<R>
where
    R: TimelineRepository,
{
    /// Sets the optional id value.
    pub fn with_id(mut self, id: Option<Id<Timeline<R::Interval>>>) -> Self {
        self.id = id;
        self
    }

    /// Executes the create timeline transaction.
    pub fn execute(self) -> Result<Timeline<R::Interval>> {
        let timeline = if let Some(timeline_id) = self.id {
            Timeline::with_id(timeline_id, self.name)
        } else {
            Timeline::new(self.name)
        };

        self.timeline_repo.create(&timeline)?;
        Ok(timeline)
    }
}

impl<R> TimelineService<R>
where
    R: TimelineRepository,
{
    pub fn create_timeline(&self, name: Name<Timeline<R::Interval>>) -> CreateTimeline<R> {
        CreateTimeline {
            timeline_repo: self.timeline_repo.clone(),
            name,
            id: Default::default(),
        }
    }
}
