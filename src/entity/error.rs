pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid entity name")]
    NotAnEntityName,
    #[error("entity already exists")]
    AlreadyExists,
    #[error("{0}")]
    Tag(#[from] crate::tag::Error),
    #[error("{0}")]
    Uuid(#[from] uuid::Error),
}
