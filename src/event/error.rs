pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("entity already exists")]
    AlreadyExists,
    #[error("event not found")]
    NotFound,
    #[error("{0}")]
    Guard(#[from] crate::guard::Error),
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
}
