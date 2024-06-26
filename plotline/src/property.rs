use std::str::FromStr;

use crate::{document::Document, name::Name};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("a property name cannot be empty or contain whitespaces")]
    NotAPropertyName,
}

/// Represents the name of a [Property].
pub struct PropertyName(String);

impl FromStr for PropertyName {
    type Err = Error;

    /// A [PropertyName] must consist of a single-world string.
    fn from_str(value: &str) -> Result<Self> {
        if value.is_empty() || value.contains(char::is_whitespace) {
            return Err(Error::NotAPropertyName);
        }

        Ok(Self(value.to_string()))
    }
}

impl AsRef<str> for PropertyName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Represents the value of a [Property].
pub enum PropertyValue {
    String(String),
    Reference(Name<Document>),
}

/// Represents an arbitrary value associated to a name.
pub struct Property {
    name: PropertyName,
    value: PropertyValue,
}
