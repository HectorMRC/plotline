use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{error::Result, Entity},
    id::Id,
};
use std::sync::Arc;

/// Implements the remove entity transaction.
pub struct RemoveEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    id: Id<Entity>,
}

impl<EntityRepo> RemoveEntity<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the remove entity transaction.
    pub fn execute(self) -> Result<()> {
        self.entity_repo.delete(self.id)
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn remove_entity(&self, id: Id<Entity>) -> RemoveEntity<EntityRepo> {
        RemoveEntity {
            entity_repo: self.entity_repo.clone(),
            id,
        }
    }
}
