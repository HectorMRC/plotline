pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    // repo
    #[error("event already exists")]
    AlreadyExists,
    #[error("event not found")]
    NotFound,
    // input
    #[error("invalid interval")]
    NotAnInterval,
    // foreign
    #[error("{0}")]
    Entity(#[from] crate::entity::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Name(#[from] crate::name::Error),
}