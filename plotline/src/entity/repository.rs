use super::{
    application::{EntityFilter, EntityRepository},
    error::{Error, Result},
    Entity,
};
use crate::{
    id::Id,
    resource::{Resource, ResourceMap},
    serde::{from_rwlock, into_rwlock},
    transaction::Tx,
};
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "from_rwlock",
        deserialize_with = "into_rwlock",
        default
    )]
    entities: RwLock<ResourceMap<Entity>>,
}

impl EntityRepository for InMemoryEntityRepository {
    type Tx = Resource<Entity>;

    fn find(&self, id: Id<Entity>) -> Result<Self::Tx> {
        self.entities
            .read()
            .map_err(Error::from)?
            .get(&id)
            .cloned()
            .map(Resource::from)
            .ok_or(Error::NotFound)
    }

    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Self::Tx>> {
        Ok(self
            .entities
            .read()
            .map_err(Error::from)?
            .values()
            .filter(|&entity| filter.matches(&entity.clone().read()))
            .cloned()
            .collect())
    }

    fn create(&self, entity: &Entity) -> Result<()> {
        let mut entities = self.entities.write().map_err(Error::from)?;

        if entities.contains_key(&entity.id) {
            return Err(Error::AlreadyExists);
        }

        entities.insert(entity.id, entity.clone().into());
        Ok(())
    }

    fn delete(&self, id: Id<Entity>) -> Result<()> {
        let mut entities = self.entities.write().map_err(Error::from)?;

        if entities.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}
