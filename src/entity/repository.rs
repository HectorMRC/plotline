use crate::id::Id;
use super::{
    error::{Error, Result},
    service::{EntityFilter, EntityRepository},
    Entity,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "into_slice_of_entities",
        deserialize_with = "from_slice_of_entities"
    )]
    entities: RwLock<HashMap<Id<Entity>, Arc<Entity>>>,
}

impl EntityRepository for InMemoryEntityRepository {
    fn find(&self, id: &Id<Entity>) -> Result<Arc<Entity>> {
        self.entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .get(id)
            .cloned()
            .ok_or(Error::NotFound)
    }

    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Arc<Entity>>> {
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

        entities.insert(entity.id, Arc::new(entity.clone()));
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

fn into_slice_of_entities<S>(
    entities: &RwLock<HashMap<Id<Entity>, Arc<Entity>>>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::Error;

    let entities = entities
        .read()
        .map_err(|err| err.to_string())
        .map_err(Error::custom)?;

    serializer.collect_seq(entities.values().map(AsRef::as_ref))
}

fn from_slice_of_entities<'de, D>(
    deserializer: D,
) -> std::result::Result<RwLock<HashMap<Id<Entity>, Arc<Entity>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RwLock::new(HashMap::from_iter(
        Vec::<Entity>::deserialize(deserializer)?
            .into_iter()
            .map(|entity| (entity.id, Arc::new(entity))),
    )))
}
