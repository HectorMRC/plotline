use crate::{
    entity::repository::InMemoryEntityRepository, event::repository::InMemoryEventRepository,
    experience::repository::InMemoryExperienceRepository, period::Period,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Implements the [Serialize] and [Deserialize] traits to persist and recover the state of the repositories.
#[derive(Serialize, Deserialize, Default)]
pub struct Snapshot<EntityRepo, EventRepo, ExperienceRepo> {
    #[serde(flatten)]
    pub entities: Arc<EntityRepo>,
    #[serde(flatten)]
    pub events: Arc<EventRepo>,
    #[serde(flatten)]
    pub experiences: Arc<ExperienceRepo>,
}

impl
    Snapshot<
        InMemoryEntityRepository,
        InMemoryEventRepository<Period<usize>>,
        InMemoryExperienceRepository<Period<usize>>,
    >
{
    /// Calls the given closure inferring all the generic types by the default ones.
    pub fn parse<D>(de: D) -> Self
    where
        D: FnOnce() -> Self,
    {
        de()
    }
}
