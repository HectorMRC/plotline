pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Experience not found")]
    NotFound,
    #[error("Multiple experiences for the same entity and event")]
    Collition,
    #[error("An experience must include at least one before or after")]
    MustBeforeOrAfter,
    #[error("The event has already been experienced by the entity")]
    EventAlreadyExperienced,
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Event(#[from] crate::event::Error),
    #[error("{0}")]
    Lock(String),
}
