use super::{TimelineRepository, TimelineService};
use crate::{
    id::Id,
    name::Name,
    timeline::{Moment, Result, Timeline, TimelineId, TimelineName},
};
use std::sync::Arc;

pub struct CreateTimeline<R> {
    timeline_repo: Arc<R>,
    name: Name<TimelineName>,
    id: Option<Id<TimelineId>>,
    moments: Vec<Moment>,
}

impl<R> CreateTimeline<R>
where
    R: TimelineRepository,
{
    pub fn execute(self) -> Result<Timeline> {
        let mut timeline = if let Some(timeline_id) = self.id {
            Timeline::with_id(timeline_id, self.name)
        } else {
            Timeline::new(self.name)
        };

        self.moments
            .into_iter()
            .try_for_each(|moment| timeline.push_moment(moment))?;

        self.timeline_repo.create(&timeline)?;
        Ok(timeline)
    }
}

impl<R> CreateTimeline<R> {
    pub fn with_id(mut self, id: Option<Id<TimelineId>>) -> Self {
        self.id = id;
        self
    }

    pub fn with_moments(mut self, moments: Vec<Moment>) -> Self {
        self.moments = moments;
        self
    }
}

impl<R> TimelineService<R>
where
    R: TimelineRepository,
{
    pub fn create(&self, name: Name<TimelineName>) -> CreateTimeline<R> {
        CreateTimeline {
            timeline_repo: self.timeline_repo.clone(),
            name,
            id: Default::default(),
            moments: Default::default(),
        }
    }
}
