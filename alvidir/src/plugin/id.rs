use std::{fmt::Display, str::FromStr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("not a plugin id")]
    NotAnId
}

/// A PluginId uniquely identifies a plugin.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PluginId(String);

impl FromStr for PluginId {
    type Err = Error;

    /// A PluginId must consist of a single word string.
    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        let is_invalid_char = |c: char| -> bool {
            !c.is_ascii() || c.is_whitespace()
        };

        if value.is_empty() || value.contains(is_invalid_char) {
            return Err(Self::Err::NotAnId);
        }

        Ok(Self(value.to_string()))
    }
}

impl Display for PluginId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for PluginId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::{PluginId, Error};
    use std::str::FromStr;

    #[test]
    fn plugin_id_from_string() {
        struct Test<'a> {
            name: &'a str,
            plugin_id: &'a str,
            must_fail: bool,
        }

        vec![
            Test {
                name: "An emty string is not a valid plugin id",
                plugin_id: "",
                must_fail: true,
            },
            Test {
                name: "An string with line feed is not a valid plugin id",
                plugin_id: "entity\nname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return is not a valid plugin id",
                plugin_id: "entity\rname",
                must_fail: true,
            },
            Test {
                name: "An string with carriage return plus line feed is not a valid plugin id",
                plugin_id: "entity\r\nname",
                must_fail: true,
            },
            Test {
                name: "An string with line feed plus carriage is not a valid plugin id",
                plugin_id: "entity\n\rname",
                must_fail: true,
            },
            Test {
                name: "A multi word single line string is not a valid plugin id",
                plugin_id: "abc 123#[]-_*&^",
                must_fail: true,
            },
            Test {
                name: "A single word non-ascii string is not a valid plugin id",
                plugin_id: "abc1234Ͼ",
                must_fail: true,
            },
            Test {
                name: "A single word ascii string is a valid plugin id",
                plugin_id: "abc123#[]-_*&^",
                must_fail: false,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let result = PluginId::from_str(test.plugin_id);
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(id) => assert_eq!(id.as_ref(), test.plugin_id, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAnId), "{}", test.name),
            }
        });
    }
}
