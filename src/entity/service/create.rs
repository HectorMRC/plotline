use super::EntityRepository;
use crate::{
    entity::{Entity, Error, Result},
    tag::Tag,
};
use std::sync::Arc;

pub struct CreateEntity<R> {
    pub entity_repo: Arc<R>,
    pub name: String,
    pub tags: Vec<String>,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let mut entity = self.name.try_into().map(Entity::new).unwrap();
        entity.tags = self
            .tags
            .into_iter()
            .map(|s| Tag::try_from(s).map_err(Error::from))
            .collect::<Result<Vec<Tag>>>()
            .unwrap();

        self.entity_repo.create(&entity).unwrap();
        Ok(entity)
    }
}

impl<R> CreateEntity<R> {
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}
