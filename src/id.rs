use serde::{Deserialize, Serialize};
use std::{fmt::Display, hash::Hash, marker::PhantomData, str::FromStr};
use crate::serde::{uuid_as_string, uuid_from_string};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("uuid: {0}")]
    Uuid(#[from] uuid::Error),
}

/// Identifiable qualifies a resource of being uniquely identifiable.
pub trait Identifiable<T> {
    fn id(&self) -> Id<T>;
}

/// An Id uniquely identifies a resource.
#[derive(Debug, Serialize, Deserialize)]
pub struct Id<T> {
    #[serde(
        serialize_with = "uuid_as_string",
        deserialize_with = "uuid_from_string"
    )]
    uuid: Uuid,

    #[serde(skip)]
    _marker: PhantomData<T>,
}

impl<T> Eq for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl<T> Copy for Id<T> {}
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid.clone(),
            _marker: PhantomData,
        }
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.uuid.hash(state);
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl<T> TryFrom<String> for Id<T> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Uuid::from_str(&value)
            .map(|uuid| Self {
                uuid,
                _marker: PhantomData,
            })
            .map_err(Error::from)
    }
}

impl<T> Id<T> {
    /// Generates a new id.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::Id;

    #[test]
    fn id_serde() {
        #[derive(Debug, PartialEq, Eq)]
        struct Any;

        let want = Id::<Any>::new();
        let yaml = serde_yaml::to_string(&want).unwrap();
        let got: Id<Any> = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(got, want, "serde ends up with different values");
    }
}
