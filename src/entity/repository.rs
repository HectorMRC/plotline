use super::{
    error::{Error, Result},
    service::{EntityFilter, EntityRepository},
    Entity,
};
use crate::{
    transaction::Resource,
    id::Id,
    serde::{hashmap_from_slice, slice_from_hashmap},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    entities: RwLock<HashMap<Id<Entity>, Arc<Mutex<Entity>>>>,
}

impl EntityRepository for InMemoryEntityRepository {
    type Tx = Resource<Entity>;

    fn find(&self, id: Id<Entity>) -> Result<Self::Tx> {
        self.entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .get(&id)
            .cloned()
            .map(Resource::from)
            .ok_or(Error::NotFound)
    }

    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Self::Tx>> {
        Ok(self
            .entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .values()
            .filter(|entity| {
                entity
                    .lock()
                    .map(|entity| filter.filter(&entity))
                    .unwrap_or_default()
            })
            .cloned()
            .map(Resource::from)
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

        entities.insert(entity.id, Arc::new(Mutex::new(entity.clone())));
        Ok(())
    }

    fn delete(&self, id: Id<Entity>) -> Result<()> {
        let mut entities = self
            .entities
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if entities.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}
