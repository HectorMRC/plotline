use crate::{
    id::{Id, Identifiable},
    transaction::Resource,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    sync::RwLock,
};

/// Serializes a hashmap into a slice of items.
pub fn slice_from_hashmap<S, T>(
    hashmap: &RwLock<HashMap<Id<T>, Resource<T>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    use serde::ser::Error;

    let hashmap = hashmap
        .read()
        .map_err(|err| err.to_string())
        .map_err(Error::custom)?;

    serializer.collect_seq(hashmap.values())
}

type Repository<T> = RwLock<HashMap<Id<T>, Resource<T>>>;

/// Deserializes an slice of [Identified] items as a hasmap indexed by the [Id] of each value.
pub fn hashmap_from_slice<'de, D, T>(deserializer: D) -> Result<Repository<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Identifiable<T>,
{
    Ok(RwLock::new(HashMap::from_iter(
        Vec::<T>::deserialize(deserializer)?
            .into_iter()
            .map(|value| (value.id(), value.into())),
    )))
}
