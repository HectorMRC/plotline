pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid entity name")]
    NotAnEntityName,
    #[error("Entity already exists")]
    AlreadyExists,
    #[error("Entity not found")]
    NotFound,
    #[error("Entity not found")]
    MoreThanOne,
    #[error("{0}")]
    Name(#[from] crate::name::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
}
