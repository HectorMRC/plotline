pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("a moment with the same name or id already exists")]
    MomentAlreadyExists,
}
