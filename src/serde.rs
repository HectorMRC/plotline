use crate::{id::Identifiable, transaction::Resource};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::HashMap, hash::Hash, sync::RwLock};

/// Serializes a hashmap into a slice of items.
pub fn slice_from_hashmap<S, K, V>(
    hashmap: &RwLock<HashMap<K, Resource<V>>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    V: Serialize,
{
    use serde::ser::Error;

    let hashmap = hashmap
        .read()
        .map_err(|err| err.to_string())
        .map_err(Error::custom)?;

    serializer.collect_seq(hashmap.values())
}

type Repository<K, V> = RwLock<HashMap<K, Resource<V>>>;

/// Deserializes an slice of [Identified] items as a hashmap indexed by the
/// [Id] of each value.
pub fn hashmap_from_slice<'de, D, K, V>(deserializer: D) -> Result<Repository<K, V>, D::Error>
where
    D: Deserializer<'de>,
    V: Deserialize<'de> + Identifiable<Id = K>,
    K: Eq + Hash,
{
    Ok(RwLock::new(HashMap::from_iter(
        Vec::<V>::deserialize(deserializer)?
            .into_iter()
            .map(|value| (value.id(), value.into())),
    )))
}
