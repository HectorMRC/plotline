pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Lock(String),
    #[error("entity already exists")]
    AlreadyExists,
}
