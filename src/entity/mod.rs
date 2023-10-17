#[cfg(feature = "cli")]
pub mod cli;
pub mod repository;
pub mod service;

mod error;
pub use error::*;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::tag::Tag;
use serde::{Deserialize, Deserializer, Serializer};
use std::hash::Hash;
use std::str::FromStr;
use uuid::Uuid;

static LINEBREAK_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\r\n|\r|\n)").unwrap());

/// Represents the universal unique id of an [Entity].
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct EntityId(
    #[serde(
        serialize_with = "uuid_as_string",
        deserialize_with = "uuid_from_string"
    )]
    Uuid,
);

fn uuid_as_string<S>(uuid: &Uuid, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

fn uuid_from_string<'de, D>(deserializer: D) -> std::result::Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    let uuid = String::deserialize(deserializer)?;
    Uuid::from_str(&uuid)
        .map_err(|err| err.to_string())
        .map_err(Error::custom)
}

impl FromStr for EntityId {
    type Err = uuid::Error;

    fn from_str(uuid: &str) -> std::result::Result<Self, Self::Err> {
        Uuid::from_str(uuid).map(Self)
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

/// Represents the unique name of an [Entity].
#[derive(Serialize, Deserialize, Clone)]
pub struct EntityName(String);

impl AsRef<str> for EntityName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EntityName {
    type Error = Error;

    /// An [Entity] name must consist of a non-empty string of alphanumeric characters.
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() || LINEBREAK_REGEX.is_match(&value) {
            return Err(Error::NotAnEntityName);
        }

        Ok(Self(value))
    }
}

/// An Entity is anything which to interact with.
#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    id: EntityId,
    name: EntityName,
    tags: Vec<Tag>,
}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Eq for Entity {}
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Entity {
    pub fn new(name: EntityName) -> Self {
        Self {
            id: EntityId::default(),
            name,
            tags: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EntityName, Error};

    #[test]
    fn an_entity_name_must_not_be_empty() {
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
            let result = EntityName::try_from(test.entity_name.to_string());
            assert_eq!(result.is_err(), test.must_fail, "{}", test.name);

            match result {
                Ok(tag) => assert_eq!(tag.as_ref(), test.entity_name, "{}", test.name),
                Err(err) => assert!(matches!(err, Error::NotAnEntityName), "{}", test.name),
            }
        });
    }
}
