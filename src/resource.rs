use crate::{
    id::Identifiable,
    transaction::{Tx, TxGuard},
};
use parking_lot::{lock_api::ArcMutexGuard, Mutex, RawMutex};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Resource implements the [Tx] trait for any piece of data.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Resource<T> {
    mu: Arc<Mutex<T>>,
}

impl<T> From<T> for Resource<T> {
    fn from(value: T) -> Self {
        Resource {
            mu: Arc::new(Mutex::new(value)),
        }
    }
}

impl<T> Tx<T> for Resource<T>
where
    T: Clone,
{
    type Guard = ResourceGuard<T>;

    fn begin(self) -> Self::Guard {
        let guard = self.mu.lock_arc();
        ResourceGuard {
            data: guard.clone(),
            guard,
        }
    }
}

impl<T> From<Arc<Mutex<T>>> for Resource<T> {
    fn from(value: Arc<Mutex<T>>) -> Self {
        Self { mu: value }
    }
}

/// ResourceGuard is the [TxGuard] implementation for [Resource].
pub struct ResourceGuard<T> {
    guard: ArcMutexGuard<RawMutex, T>,
    data: T,
}

impl<T> Deref for ResourceGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ResourceGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> TxGuard<T> for ResourceGuard<T> {
    fn commit(mut self) {
        *self.guard = self.data;
    }
}

/// A ResourceMap is a [HashMap] of [Identifiable] [Resource]s.
pub struct ResourceMap<T>
where
    T: Identifiable,
{
    resources: HashMap<T::Id, Resource<T>>,
}

impl<T> Deref for ResourceMap<T>
where
    T: Identifiable,
{
    type Target = HashMap<T::Id, Resource<T>>;

    fn deref(&self) -> &Self::Target {
        &self.resources
    }
}

impl<T> DerefMut for ResourceMap<T>
where
    T: Identifiable,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.resources
    }
}

impl<T> Default for ResourceMap<T>
where
    T: Identifiable,
{
    fn default() -> Self {
        Self {
            resources: Default::default(),
        }
    }
}

impl<T> Serialize for ResourceMap<T>
where
    T: Identifiable + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_seq(self.values())
    }
}

impl<'a, T> Deserialize<'a> for ResourceMap<T>
where
    T: Identifiable + Deserialize<'a>,
    T::Id: Hash,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'a>,
    {
        Ok(ResourceMap {
            resources: HashMap::from_iter(
                Vec::<T>::deserialize(deserializer)?
                    .into_iter()
                    .map(|value| (value.id(), value.into())),
            ),
        })
    }
}
