use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{error::Result, Entity},
    id::Identifiable,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    id: <Entity as Identifiable>::Id,
}

impl<EntityRepo> FindEntity<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the find query, through which one, and exactly one, entity must
    /// be retrived.
    pub async fn execute(self) -> Result<Entity> {
        Ok(self.entity_repo.find(self.id).await?.read().await.clone())
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn find_entity(&self, id: <Entity as Identifiable>::Id) -> FindEntity<EntityRepo> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            id,
        }
    }
}
