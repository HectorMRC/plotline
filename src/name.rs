use once_cell::sync::Lazy;
use regex::Regex;
use std::{fmt::Display, marker::PhantomData};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid name")]
    NotAName,
}

/// Matches any combination of line-break characters.
static LINEBREAK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\r\n|\r|\n)").unwrap());

/// An Name identifies one or more resources.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Name<T> {
    name: String,

    #[serde(skip)]
    _type: PhantomData<T>,
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

    /// A name must consist of a non-empty and single line string.
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() || LINEBREAK_REGEX.is_match(&value) {
            return Err(Error::NotAName);
        }

        Ok(Self {
            name: value,
            _type: PhantomData,
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
