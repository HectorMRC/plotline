use super::{EntityRepository, EntityService};
use crate::{
    entity::{error::Error, Entity, EntityID, EntityName, Result},
    tag::Tags,
};
use std::{marker::PhantomData, sync::Arc};

pub struct CreateEntity<R> {
    entity_repo: Arc<R>,
    name: EntityName,
    id: Option<EntityID>,
    tags: Tags,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let mut entity = if let Some(entity_id) = self.id {
            Entity::with_id(entity_id, self.name)
        } else {
            Entity::new(self.name)
        };

        entity.tags = self.tags;

        self.entity_repo.create(&entity)?;
        Ok(entity)
    }
}

impl<R> CreateEntity<R> {
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
    pub fn create(&self, name: EntityName) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            name,
            id: Default::default(),
            tags: Default::default(),
        }
    }
}
