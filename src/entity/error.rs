pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid entity name")]
    NotAnEntityName,
    #[error("entity already exists")]
    AlreadyExists,
    #[error("entity not found")]
    NotFound,
    #[error("lock: {0}")]
    Lock(String),
    #[error("name: {0}")]
    Name(#[from] crate::name::Error),
    #[error("tag: {0}")]
    Tag(#[from] crate::tag::Error),
    #[error("id: {0}")]
    Id(#[from] crate::id::Error),
}
