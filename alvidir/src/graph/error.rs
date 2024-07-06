pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("a graph cannot contain two or more nodes with the same id")]
    DuplicatedId,
}
