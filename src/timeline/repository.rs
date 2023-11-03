use super::Timeline;
use crate::id::Id;
use std::collections::HashMap;

pub struct InMemoryTimelineRepository {
    timelines: HashMap<Id<Timeline>, Timeline>,
}
