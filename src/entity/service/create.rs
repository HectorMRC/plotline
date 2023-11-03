use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, Result},
    id::Id,
    name::Name,
};
use std::sync::Arc;

/// Implements the create entity transaction.
pub struct CreateEntity<R> {
    entity_repo: Arc<R>,
    name: Name<Entity>,
    id: Option<Id<Entity>>,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    /// Executes the create entity transaction.
    pub fn execute(self) -> Result<Entity> {
        let mut entity = if let Some(entity_id) = self.id {
            Entity::with_id(entity_id, self.name)
        } else {
            Entity::new(self.name)
        };

        self.entity_repo.create(&entity)?;
        Ok(entity)
    }
}

impl<R> CreateEntity<R> {
    pub fn with_id(mut self, id: Option<Id<Entity>>) -> Self {
        self.id = id;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn create_entity(&self, name: Name<Entity>) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            name,
            id: Default::default(),
        }
    }
}
