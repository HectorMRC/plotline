use crate::entity::repository::InMemoryEntityRepository;
use std::sync::Arc;

/// Implements the [Serialize] and [Deserialize] traits to persist and recover the state of the repositories.
#[derive(Serialize, Deserialize)]
pub struct Snapshot<E> {
    pub entities: Arc<E>,
}

impl Snapshot<InMemoryEntityRepository> {
    /// Calls the given closure inferring all the generic types by the default ones.
    pub fn parse<D, E>(de: D) -> Result<Self, E>
    where
        D: FnOnce() -> Result<Self, E>,
    {
        de()
    }
}
