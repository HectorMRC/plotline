use std::str::FromStr;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("a tag cannot be empty or contain whitespaces")]
    NotATag,
}

/// Represents a single-world string.
#[derive(Clone)]
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

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::tag::{Error, Tag};

    #[test]
    fn tag_from_string() {
        struct Test<'a> {
            name: &'a str,
            tag_value: &'a str,
            must_fail: bool,
        }

        vec![
            Test {
                name: "An emty string is not a valid tag",
                tag_value: "",
                must_fail: true,
            },
            Test {
                name: "An string with line feed is not a valid tag",
                tag_value: "entity\nname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return is not a valid tag",
                tag_value: "entity\rname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return plus line feed is not a valid tag",
                tag_value: "entity\r\nname",
                must_fail: true,
            },
            Test {
                name: "An string with line feed plus carriage is not a valid tag",
                tag_value: "entity\n\rname",
                must_fail: true,
            },
            Test {
                name: "A multi word single line string is not a valid tag",
                tag_value: "abc 123#[]-_*&^",
                must_fail: true,
            },
            Test {
                name: "A single word string is a valid tag",
                tag_value: "abc123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = Tag::from_str(test.tag_value);
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(name) => assert_eq!(name.as_ref(), test.tag_value, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotATag), "{}", test.name),
            }
        });
    }
}
