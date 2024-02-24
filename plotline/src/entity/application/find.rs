use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{error::Result, Entity},
    id::Id,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    id: Id<Entity>,
}

impl<EntityRepo> FindEntity<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the find query, through which one, and exactly one, entity must
    /// be retrived.
    pub fn execute(self) -> Result<Entity> {
        Ok(self.entity_repo.find(self.id)?.read().clone())
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn find_entity(&self, id: Id<Entity>) -> FindEntity<EntityRepo> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            id,
        }
    }
}
