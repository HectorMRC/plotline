pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    // repo
    #[error("event already exists")]
    AlreadyExists,
    #[error("event not found")]
    NotFound,
    // input
    #[error("event name must be set")]
    NameRequired,
    #[error("event interval must be set")]
    IntervalRequired,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
    #[error("{0}")]
    Custom(&'static str),
}
