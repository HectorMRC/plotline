//! Definitions for creating and managing arbitrary names.

use std::{fmt::Display, hash::Hash, marker::PhantomData, str::FromStr};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("a name cannot be empty or contain more than one line")]
    NotAName,
}

/// Represents a single-line string that identifies one or more resources.
#[derive(Debug, Serialize, Deserialize)]
pub struct Name<T> {
    value: String,
    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> FromStr for Name<T> {
    type Err = Error;

    /// A name must consist of a single line string.
    fn from_str(value: &str) -> Result<Self> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 2] = ['\n', '\r'];
            INVALID_CHARS.contains(&c)
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAName);
        }

        Ok(Self {
            value: value.to_string(),
            _marker: PhantomData,
        })
    }
}

impl<T> AsRef<str> for Name<T> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<T> Display for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl<T> Clone for Name<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Hash for Name<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> Eq for Name<T> {}
impl<T> PartialEq for Name<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialEq<&str> for Name<T> {
    fn eq(&self, other: &&str) -> bool {
        &self.value == other
    }
}

impl<T> Name<T> {
    /// Returns a new name with the same value as the given one.
    pub fn from<U>(name: Name<U>) -> Self {
        Self {
            value: name.value,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::{Error, Name};

    #[test]
    fn name_from_string() {
        struct Test<'a> {
            name: &'a str,
            name_value: &'a str,
            must_fail: bool,
        }

        vec![
            Test {
                name: "An emty string is not a valid name",
                name_value: "",
                must_fail: true,
            },
            Test {
                name: "An string with line feed is not a valid name",
                name_value: "entity\nname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return is not a valid name",
                name_value: "entity\rname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return plus line feed is not a valid name",
                name_value: "entity\r\nname",
                must_fail: true,
            },
            Test {
                name: "An string with line feed plus carriage is not a valid name",
                name_value: "entity\n\rname",
                must_fail: true,
            },
            Test {
                name: "A multi word single line string is a valid name",
                name_value: "abc 123#[]-_*&^",
                must_fail: false,
            },
            Test {
                name: "A single word string is a valid name",
                name_value: "abc123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = Name::<()>::from_str(test.name_value);
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(name) => assert_eq!(name.as_ref(), test.name_value, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAName), "{}", test.name),
            }
        });
    }
}
