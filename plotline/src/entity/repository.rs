use super::{
    application::{EntityFilter, EntityRepository},
    error::{Error, Result},
    Entity,
};
use crate::{
    id::Id,
    macros::equals_or_return,
    resource::{from_rwlock, into_rwlock, Resource, ResourceMap},
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

    async fn find(&self, id: Id<Entity>) -> Result<Self::Tx> {
        self.entities
            .read()?
            .get(&id)
            .cloned()
            .map(Resource::from)
            .ok_or(Error::NotFound)
    }

    async fn filter(&self, filter: &EntityFilter) -> Result<Vec<Self::Tx>> {
        let entities: Vec<_> = self
            .entities
            .read()
            .map(|entities| entities.values().cloned().collect())?;

        let mut matches = Vec::new();
        for entity_tx in entities {
            let experience = entity_tx.read().await;
            if filter.matches(&experience) {
                matches.push(entity_tx.clone());
            }
        }

        Ok(matches)
    }

    async fn create(&self, entity: &Entity) -> Result<()> {
        let mut entities = self.entities.write().map_err(Error::from)?;

        if entities.contains_key(&entity.id) {
            return Err(Error::AlreadyExists);
        }

        entities.insert(entity.id, entity.clone().into());
        Ok(())
    }

    async fn delete(&self, id: Id<Entity>) -> Result<()> {
        let mut entities = self.entities.write().map_err(Error::from)?;

        if entities.remove(&id).is_none() {
            return Err(Error::NotFound);
        }

        Ok(())
    }
}

impl EntityFilter {
    fn matches(&self, entity: &Entity) -> bool {
        equals_or_return!(self.name, &entity.name);
        equals_or_return!(self.id, &entity.id);
        true
    }
}
