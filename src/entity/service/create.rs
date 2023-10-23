use std::sync::Arc;

use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, EntityID, Result},
    tag::Tags,
};

pub struct CreateEntity<R> {
    entity_repo: Arc<R>,
    name: String,
    id: Option<EntityID>,
    tags: Tags,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let entity_name = self.name.try_into()?;
        let mut entity = if let Some(entity_id) = self.id {
            Entity::with_id(entity_id, entity_name)
        } else {
            Entity::new(entity_name)
        };

        entity.tags = self.tags;

        self.entity_repo.create(&entity)?;
        Ok(entity)
    }
}

impl<R> CreateEntity<R> {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_id(mut self, id: Option<EntityID>) -> Self {
        self.id = id;
        self
    }

    pub fn with_tags(mut self, tags: Tags) -> Self {
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
            name: Default::default(),
            id: Default::default(),
            tags: Default::default(),
        }
    }
}
