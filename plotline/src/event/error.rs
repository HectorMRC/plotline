pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    // repo
    #[error("event already exists")]
    AlreadyExists,
    #[error("event not found")]
    NotFound,
    // input
    #[error("an event name cannot be empty")]
    EmptyName,
    #[error("an event interval cannot be empty")]
    EmptyInterval,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
}
