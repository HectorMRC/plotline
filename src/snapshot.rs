use crate::entity::repository::InMemoryEntityRepository;
use std::sync::Arc;

/// Implements the [Serialize] and [Deserialize] traits to persist and recover the state of the repositories.
#[derive(Serialize, Deserialize, Default)]
pub struct Snapshot<E> {
    #[serde(flatten)]
    pub entities: Arc<E>,
}

impl Snapshot<InMemoryEntityRepository> {
    /// Calls the given closure inferring all the generic types by the default ones.
    pub fn parse<D>(de: D) -> Self
    where
        D: FnOnce() -> Self,
    {
        de()
    }
}
