use super::{
    error::{Error, Result},
    service::EntityRepository,
    Entity,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{
    collections::HashSet,
    sync::{Arc, RwLock},
};

#[derive(Default, Serialize, Deserialize)]
pub struct InMemoryEntityRepository {
    #[serde(
        serialize_with = "into_slice_of_entities",
        deserialize_with = "from_slice_of_entities"
    )]
    entities: RwLock<HashSet<Arc<Entity>>>,
}

impl EntityRepository for InMemoryEntityRepository {
    fn create(&self, entity: &Entity) -> Result<()> {
        let mut entities = self.entities.write().unwrap();
        if entities.get(entity).is_some() {
            return Err(Error::AlreadyExists);
        }

        entities.insert(Arc::new(entity.clone()));
        Ok(())
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
