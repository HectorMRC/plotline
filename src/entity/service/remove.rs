use super::{EntityRepository, EntityService};
use crate::{
    entity::{error::Result, Entity},
    id::Id,
};
use std::sync::Arc;

/// Implements the remove entity transaction.
pub struct RemoveEntity<R> {
    entity_repo: Arc<R>,
    id: Id<Entity>,
}

impl<R> RemoveEntity<R>
where
    R: EntityRepository,
{
    /// Executes the remove entity transaction.
    pub fn execute(self) -> Result<()> {
        self.entity_repo.delete(self.id)
    }
}

impl<R> RemoveEntity<R> {
    pub fn with_id(mut self, id: Id<Entity>) -> Self {
        self.id = id;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn remove_entity(&self, id: Id<Entity>) -> RemoveEntity<R> {
        RemoveEntity {
            entity_repo: self.entity_repo.clone(),
            id,
        }
    }
}
