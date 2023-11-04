use super::{
    error::{Error, Result},
    service::{EntityFilter, EntityRepository},
    Entity,
};
use crate::{
    id::Id,
    serde::{hashmap_from_slice, slice_from_hashmap},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    entities: RwLock<HashMap<Id<Entity>, Entity>>,
}

impl EntityRepository for InMemoryEntityRepository {
    fn find(&self, id: &Id<Entity>) -> Result<Entity> {
        self.entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .get(id)
            .cloned()
            .ok_or(Error::NotFound)
    }

    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Entity>> {
        Ok(self
            .entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .values()
            .filter(|entity| filter.filter(entity))
            .cloned()
            .collect())
    }

    fn create(&self, entity: &Entity) -> Result<()> {
        let mut entities = self
            .entities
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if entities.contains_key(&entity.id) {
            return Err(Error::AlreadyExists);
        }

        entities.insert(entity.id, entity.clone());
        Ok(())
    }

    fn delete(&self, entity: &Entity) -> Result<()> {
        let mut entities = self
            .entities
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if entities.remove(&entity.id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}
