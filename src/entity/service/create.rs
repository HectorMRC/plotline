use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, EntityID, EntityName, Result},
    id::ID,
    name::Name,
    tag::Tags,
};
use std::sync::Arc;

pub struct CreateEntity<R> {
    entity_repo: Arc<R>,
    name: Name<EntityName>,
    id: Option<ID<EntityID>>,
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
    pub fn with_id(mut self, id: Option<ID<EntityID>>) -> Self {
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
    pub fn create(&self, name: Name<EntityName>) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            name,
            id: Default::default(),
            tags: Default::default(),
        }
    }
}
