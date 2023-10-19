use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, Error, Result},
    tag::Tag,
};
use std::sync::Arc;

pub struct CreateEntity<R> {
    entity_repo: Arc<R>,
    id: String,
    name: String,
    tags: Vec<String>,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let entity_id = self.id.try_into()?;
        let entity_name = self.name.try_into()?;
        let mut entity = Entity::new(entity_id, entity_name);

        entity.tags = self
            .tags
            .into_iter()
            .map(|s| Tag::try_from(s).map_err(Error::from))
            .collect::<Result<Vec<Tag>>>()?;

        self.entity_repo.create(&entity)?;
        Ok(entity)
    }
}

impl<R> CreateEntity<R> {
    pub fn with_id(mut self, id: String) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn create(&self) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            id: Default::default(),
            name: Default::default(),
            tags: Default::default(),
        }
    }
}
