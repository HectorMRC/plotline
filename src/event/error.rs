pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entity already exists")]
    AlreadyExists,
    #[error("event not found")]
    NotFound,
    #[error("event ID must be set")]
    IdRequired,
    #[error("event name must be set")]
    NameRequired,
    #[error("event interval must be set")]
    IntervalRequired,
    #[error("{0}")]
    Transaction(#[from] crate::transaction::Error),
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
    #[error("{0}")]
    Custom(&'static str),
}
