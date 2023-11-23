pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Event not found")]
    NotFound,
    #[error("Event ID must be set")]
    IdRequired,
    #[error("Event name must be set")]
    NameRequired,
    #[error("Event interval must be set")]
    IntervalRequired,
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
    #[error("{0}")]
    Custom(&'static str),
}
