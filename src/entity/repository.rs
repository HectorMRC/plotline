use super::{
    error::{Error, Result},
    service::EntityRepository,
    Entity, EntityName,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "into_slice_of_entities",
        deserialize_with = "from_slice_of_entities"
    )]
    entities: RwLock<HashSet<Arc<Entity>>>,
}

impl EntityRepository for InMemoryEntityRepository {
    fn list(&self) -> Result<Vec<Arc<Entity>>> {
        Ok(self
            .entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .iter()
            .cloned()
            .collect())
    }

    fn find_by_name(&self, name: &EntityName) -> Result<Arc<Entity>> {
        self.entities
            .read()
            .map_err(|err| Error::Lock(err.to_string()))?
            .iter()
            .find(|entity| entity.name() == name)
            .cloned()
            .ok_or(Error::NotFound)
    }

    fn create(&self, entity: &Entity) -> Result<()> {
        let mut entities = self
            .entities
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if entities.get(entity).is_some() {
            return Err(Error::AlreadyExists);
        }

        entities.insert(Arc::new(entity.clone()));
        Ok(())
    }

    fn remove(&self, entity: &Entity) -> Result<()> {
        let mut entities = self
            .entities
            .write()
            .map_err(|err| Error::Lock(err.to_string()))?;

        if entities.remove(entity) {
            return Ok(());
        }

        Err(Error::NotFound)
    }
}

fn into_slice_of_entities<S>(
    entities: &RwLock<HashSet<Arc<Entity>>>,
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

    serializer.collect_seq(entities.iter().map(AsRef::as_ref))
}

fn from_slice_of_entities<'de, D>(
    deserializer: D,
) -> std::result::Result<RwLock<HashSet<Arc<Entity>>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(RwLock::new(HashSet::from_iter(
        Vec::<Entity>::deserialize(deserializer)?
            .into_iter()
            .map(|entity| Arc::new(entity)),
    )))
}
