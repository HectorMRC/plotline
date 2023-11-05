use super::{service::TimelineRepository, Error, Timeline};
use crate::{
    id::Id,
    serde::{hashmap_from_slice, slice_from_hashmap}, interval::Interval,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default, Serialize, Deserialize)]
pub struct InMemoryTimelineRepository<I>
where I: Interval + Serialize + for<'a> Deserialize<'a> {
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    timelines: RwLock<HashMap<Id<Timeline<I>>, Timeline<I>>>,
}

impl<I> TimelineRepository for InMemoryTimelineRepository<I>
where I: Interval + Serialize + for<'a> Deserialize<'a> {
    type Interval = I;

    fn create(&self, timeline: &Timeline<Self::Interval>) -> super::Result<()> {
        let mut timelines = self
            .timelines
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if timelines.contains_key(&timeline.id)
            || timelines.values().any(|t| t.name == timeline.name)
        {
            return Err(Error::AlreadyExists);
        }

        timelines.insert(timeline.id, timeline.clone());
        Ok(())
    }
}
