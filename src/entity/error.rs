pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // repo
    #[error("entity already exists")]
    AlreadyExists,
    #[error("entity not found")]
    NotFound,
    // foreign
    #[error("{0}")]
    Name(#[from] crate::name::Error),
    #[error("{0}")]
    Id(#[from] crate::id::Error),
    #[error("{0}")]
    Lock(String),
}
