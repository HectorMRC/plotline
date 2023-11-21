pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Experience not found")]
    NotFound,
    #[error("Multiple experiences with entity_id = {entity:?} and event_id = {event:?}")]
    Collition { entity: String, event: String },
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Transaction(#[from] crate::transaction::Error),
    #[error("{0}")]
    Lock(String),
}
