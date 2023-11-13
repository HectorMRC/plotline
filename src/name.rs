use serde::{Deserialize, Serialize};
use std::{fmt::Display, marker::PhantomData};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid name")]
    NotAName,
}

/// Returns true if, and only if, the given char c is an invalid character inside a name. 
fn is_invalid_char(c: char) -> bool {
    const INVALID_CHARS: [char;3] = ['\n', '\r', ' '];
    INVALID_CHARS.contains(&c)
}

/// An Name identifies one or more resources.
#[derive(Serialize, Deserialize, Clone)]
pub struct Name<T> {
    name: String,

    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Eq for Name<T> {}
impl<T> PartialEq for Name<T> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<T> AsRef<str> for Name<T> {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl<T> Display for Name<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}


impl<T> TryFrom<String> for Name<T> {
    type Error = Error;

    /// A name must consist of a single word string.
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAName);
        }

        Ok(Self {
            name: value,
            _marker: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Name};

    #[test]
    fn name_from_string() {
        struct Test<'a> {
            name: &'a str,
            entity_name: &'a str,
            must_fail: bool,
        }

        vec![
            Test {
                name: "An emty string is not a valid name",
                entity_name: "",
                must_fail: true,
            },
            Test {
                name: "An string with line feed is not a valid name",
                entity_name: "entity\nname",
                must_fail: true,
            },
            Test {
                name: "An string with carreiage return is not a valid name",
                entity_name: "entity\rname",
                must_fail: true,
            },
            Test {
                name: "An string with carreiage return plus line feed is not a valid name",
                entity_name: "entity\r\nname",
                must_fail: true,
            },
            Test {
                name: "A non-empty single line string is a valid name",
                entity_name: "abc 123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            struct Any;

            let result = Name::<Any>::try_from(test.entity_name.to_string());
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(tag) => assert_eq!(tag.as_ref(), test.entity_name, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAName), "{}", test.name),
            }
        });
    }
}
