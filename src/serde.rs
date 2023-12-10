use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::sync::RwLock;

/// Serializes the serializable content of a [RwLock].
pub fn from_rwlock<S, T>(rwlock: &RwLock<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    use serde::ser::Error;

    rwlock
        .read()
        .map_err(|err| err.to_string())
        .map_err(Error::custom)?
        .serialize(serializer)
}

/// Deserializes the deserializable content into a [RwLock].
pub fn into_rwlock<'de, D, T>(deserializer: D) -> Result<RwLock<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(RwLock::new(T::deserialize(deserializer)?))
}
