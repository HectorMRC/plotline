use crate::{
    entity::repository::InMemoryEntityRepository, timeline::{repository::InMemoryTimelineRepository, Period},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Implements the [Serialize] and [Deserialize] traits to persist and recover the state of the repositories.
#[derive(Serialize, Deserialize, Default)]
pub struct Snapshot<E, T> {
    #[serde(flatten)]
    pub entities: Arc<E>,
    #[serde(flatten)]
    pub timelines: Arc<T>,
}

impl Snapshot<
    InMemoryEntityRepository,
    InMemoryTimelineRepository<Period<usize>>
> {
    /// Calls the given closure inferring all the generic types by the default ones.
    pub fn parse<D>(de: D) -> Self
    where
        D: FnOnce() -> Self,
    {
        de()
    }
}
