pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid attribute key")]
    NotAnAttributeKey,
    #[error("invalid attribute value")]
    NotAnAttributeValue,
}
