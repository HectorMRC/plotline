use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Display, hash::Hash, marker::PhantomData, str::FromStr};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("uuid: {0}")]
    Uuid(#[from] uuid::Error),
}

/// Identifiable qualifies a resource of being uniquely identifiable.
pub trait Identifiable<T> {
    fn id(&self) -> Id<T>;
}

/// An Id uniquely identifies a resource.
#[derive(Debug)]
pub struct Id<T> {
    uuid: Uuid,
    _marker: PhantomData<T>,
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.uuid.to_string())
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let uuid = String::deserialize(deserializer)?;
        Uuid::from_str(&uuid)
            .map(|uuid| Self {
                uuid,
                _marker: PhantomData,
            })
            .map_err(|err| err.to_string())
            .map_err(Error::custom)
    }
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
        *self
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.uuid.cmp(&other.uuid)
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

impl<T> Default for Id<T> {
    /// Returns a random generated id.
    fn default() -> Self {
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
        let want: Id<()> = Default::default();
        let yaml = serde_yaml::to_string(&want).unwrap();
        let got: Id<()> = serde_yaml::from_str(&yaml).unwrap();

        assert_eq!(got, want, "serde ends up with different values");
    }
}
