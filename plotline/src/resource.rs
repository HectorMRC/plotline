use crate::{
    id::Identifiable,
    transaction::{Tx, TxReadGuard, TxWriteGuard},
};
use parking_lot::{
    lock_api::{ArcRwLockReadGuard, ArcRwLockWriteGuard},
    RawRwLock, RwLock,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("there cannot be two or more resources with the same id")]
    DuplicatedId,
}

/// Resource implements the [Tx] trait for any piece of data.
#[derive(Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Resource<T> {
    lock: Arc<RwLock<T>>,
}

impl<T> From<T> for Resource<T> {
    fn from(value: T) -> Self {
        Resource {
            lock: Arc::new(RwLock::new(value)),
        }
    }
}

impl<T> Tx<T> for Resource<T>
where
    T: Clone,
{
    type ReadGuard = ResourceReadGuard<T>;
    type WriteGuard = ResourceWriteGuard<T>;

    fn read(self) -> Self::ReadGuard {
        ResourceReadGuard {
            guard: self.lock.read_arc(),
        }
    }

    fn write(self) -> Self::WriteGuard {
        let guard = self.lock.write_arc();
        ResourceWriteGuard {
            data: guard.clone(),
            guard,
        }
    }
}

impl<T> From<Arc<RwLock<T>>> for Resource<T> {
    fn from(value: Arc<RwLock<T>>) -> Self {
        Self { lock: value }
    }
}

/// ResourceReadGuard is the [TxReadGuard] implementation for [Resource].
pub struct ResourceReadGuard<T> {
    guard: ArcRwLockReadGuard<RawRwLock, T>,
}

impl<T> Deref for ResourceReadGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> TxReadGuard<T> for ResourceReadGuard<T> {
    fn release(self) {}
}

/// ResourceWriteGuard is the [TxWriteGuard] implementation for [Resource].
pub struct ResourceWriteGuard<T> {
    guard: ArcRwLockWriteGuard<RawRwLock, T>,
    data: T,
}

impl<T> Deref for ResourceWriteGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for ResourceWriteGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> TxWriteGuard<T> for ResourceWriteGuard<T> {
    fn commit(mut self) {
        *self.guard = self.data;
    }

    fn rollback(self) {}
}

/// A ResourceMap is a collection of [Identifiable] [Resource]s.
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

impl<T> ResourceMap<T>
where
    T: Identifiable,
{
    pub fn new(resources: HashMap<T::Id, Resource<T>>) -> Self {
        Self { resources }
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
        Vec::<T>::deserialize(deserializer)?
            .into_iter()
            .try_fold(HashMap::new(), |mut resources, entry| {
                resources
                    .insert(entry.id(), entry.into())
                    .is_none()
                    .then_some(resources)
                    .ok_or(Error::DuplicatedId)
            })
            .map(Self::new)
            .map_err(serde::de::Error::custom)
    }
}
