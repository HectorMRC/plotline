use serde::{de::Deserializer, Deserialize, Serialize, Serializer};
use std::{fmt::Display, marker::PhantomData, str::FromStr};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("uuid: {0}")]
    Uuid(#[from] uuid::Error),
}

/// An Id uniquely identifies a resource.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
pub struct Id<T> {
    #[serde(
        serialize_with = "uuid_as_string",
        deserialize_with = "uuid_from_string"
    )]
    uuid: Uuid,

    #[serde(skip)]
    _type: PhantomData<T>,
}

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
                _type: PhantomData,
            })
            .map_err(Error::from)
    }
}

impl<T> Id<T> {
    /// Generates a new id.
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            _type: PhantomData,
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
