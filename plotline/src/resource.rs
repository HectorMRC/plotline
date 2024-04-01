use crate::{
    id::Identifiable,
    transaction::{Tx, TxReadGuard, TxWriteGuard},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult},
};

/// Given a [LockResult] return the inner value no matter it has been poisoned
/// or not.
#[inline]
pub fn infallible_lock<T>(result: LockResult<T>) -> T {
    match result {
        Ok(inner) => inner,
        Err(error) => error.into_inner(),
    }
}

/// Serializes the serializable content from a [RwLock].
pub fn from_rwlock<S, T>(rwlock: &RwLock<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    use serde::ser::Error;

    rwlock.read().map_err(Error::custom)?.serialize(serializer)
}

/// Deserializes the deserializable content into a [RwLock].
pub fn into_rwlock<'de, D, T>(deserializer: D) -> Result<RwLock<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(RwLock::new(T::deserialize(deserializer)?))
}

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
    type ReadGuard<'a> = ResourceReadGuard<'a, T> where T: 'a;
    type WriteGuard<'a> = ResourceWriteGuard<'a, T> where T: 'a;

    async fn read(&self) -> Self::ReadGuard<'_> {
        ResourceReadGuard {
            guard: infallible_lock(self.lock.read()),
        }
    }

    async fn write(&self) -> Self::WriteGuard<'_> {
        let guard = infallible_lock(self.lock.write());

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
pub struct ResourceReadGuard<'a, T> {
    guard: RwLockReadGuard<'a, T>,
}

impl<'a, T> Deref for ResourceReadGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<'a, T> TxReadGuard<T> for ResourceReadGuard<'a, T> {
    fn release(self) {}
}

/// ResourceWriteGuard is the [TxWriteGuard] implementation for [Resource].
pub struct ResourceWriteGuard<'a, T> {
    guard: RwLockWriteGuard<'a, T>,
    data: T,
}

impl<'a, T> Deref for ResourceWriteGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> DerefMut for ResourceWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<'a, T> TxWriteGuard<T> for ResourceWriteGuard<'a, T> {
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

impl<T> ResourceMap<T>
where
    T: Identifiable,
{
    pub fn new(resources: HashMap<T::Id, Resource<T>>) -> Self {
        Self { resources }
    }
}
