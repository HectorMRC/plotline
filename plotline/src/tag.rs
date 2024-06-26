use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("a tag cannot be empty or contain whitespaces")]
    NotATag,
}

/// A single-world string.
pub struct Tag(String);

impl FromStr for Tag {
    type Err = Error;

    /// A [Tag] must consist of a single-world string.
    fn from_str(value: &str) -> Result<Self> {
        if value.is_empty() || value.contains(char::is_whitespace) {
            return Err(Error::NotATag);
        }

        Ok(Self(value.to_string()))
    }
}
