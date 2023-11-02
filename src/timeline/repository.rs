use super::{Timeline, TimelineId};
use crate::id::Id;
use std::collections::HashMap;

pub struct InMemoryTimelineRepository {
    timelines: HashMap<Id<TimelineId>, Timeline>,
}
