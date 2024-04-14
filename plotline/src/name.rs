use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, marker::PhantomData, str::FromStr};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("invalid name")]
    NotAName,
}

/// A Name identifies one or more resources.
#[derive(Debug, Clone)]
pub struct Name<T> {
    name: String,
    _marker: PhantomData<T>,
}

impl<T> Serialize for Name<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}

impl<'de, T> Deserialize<'de> for Name<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        String::deserialize(deserializer)
            .map(|name| Self {
                name,
                _marker: PhantomData,
            })
            .map_err(Error::custom)
    }
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

    fn try_from(value: String) -> Result<Self> {
        Name::from_str(&value)
    }
}

impl<T> FromStr for Name<T> {
    type Err = Error;

    /// A name must consist of a single word string.
    fn from_str(value: &str) -> Result<Self> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 2] = ['\n', '\r'];
            INVALID_CHARS.contains(&c)
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Error::NotAName);
        }

        Ok(Self {
            name: value.to_string(),
            _marker: PhantomData,
        })
    }
}

impl<T> Default for Name<T> {
    /// Returns the "no name" name.
    fn default() -> Self {
        Self {
            name: "no name".to_string(),
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
                name: "A multi word single line string is a valid name",
                entity_name: "abc 123#[]-_*&^",
                must_fail: false,
            },
            Test {
                name: "A single word string is a valid name",
                entity_name: "abc123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = Name::<()>::from_str(test.entity_name);
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(tag) => assert_eq!(tag.as_ref(), test.entity_name, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAName), "{}", test.name),
            }
        });
    }
}
