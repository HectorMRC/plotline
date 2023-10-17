mod error;
pub use error::*;

/// Contains all the space-like characters that are available to use in tags
const SEPARATORS: [char; 2] = ['_', '-'];

/// In very few words a Tag represents a concept.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag(String);

impl AsRef<str> for Tag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Tag {
    type Error = Error;

    /// A [Tag] must consist of a non-empty string of alphanumeric characters.
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty()
            || value
                .chars()
                .any(|c| !c.is_alphanumeric() && !SEPARATORS.contains(&c))
        {
            return Err(Error::NotATag);
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Tag};

    #[test]
    fn a_tag_must_consist_of_alphanumeric_characters() {
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
                name: "A string with spaces is not a valid tag",
                tag_value: "a b",
                must_fail: true,
            },
            Test {
                name: "A string with line breaks is not a valid tag",
                tag_value: "a\nb",
                must_fail: true,
            },
            Test {
                name: "An alphanumeric string is a valid tag",
                tag_value: "abc123",
                must_fail: false,
            },
            Test {
                name: "A string with underscores is a valid tag",
                tag_value: "abc_123",
                must_fail: false,
            },
            Test {
                name: "A string with hyphen is a valid tag",
                tag_value: "abc-123",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = Tag::try_from(test.tag_value.to_string());
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(tag) => assert_eq!(tag.as_ref(), test.tag_value, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotATag), "{}", test.name),
            }
        });
    }
}
