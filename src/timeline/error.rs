pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("timeline already exists")]
    AlreadyExists,
    #[error("timeline not found")]
    NotFound,
    #[error("{0}")]
    Lock(String),
}
