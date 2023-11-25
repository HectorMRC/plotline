use super::{
    application::{EntityFilter, EntityRepository},
    error::{Error, Result},
    Entity,
};
use crate::{
    id::Id,
    serde::{hashmap_from_slice, slice_from_hashmap},
    transaction::{Resource, Tx},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::RwLock};

type Repository<T> = RwLock<HashMap<Id<T>, Resource<T>>>;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "slice_from_hashmap",
        deserialize_with = "hashmap_from_slice",
        default
    )]
    entities: Repository<Entity>,
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
            .filter(|&entity| filter.filter(&entity.clone().begin()))
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

        entities.insert(entity.id, entity.clone().into());
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
