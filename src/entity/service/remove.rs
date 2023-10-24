use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity, EntityID};
use std::sync::Arc;

pub struct RemoveEntity<R> {
    entity_repo: Arc<R>,
    id: EntityID,
}

impl<R> RemoveEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Arc<Entity>> {
        let entity = self.entity_repo.find(&self.id)?;
        self.entity_repo.remove(&entity).map(|_| entity)
    }
}

impl<R> RemoveEntity<R> {
    pub fn with_id(mut self, id: EntityID) -> Self {
        self.id = id;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn remove(&self, id: EntityID) -> RemoveEntity<R> {
        RemoveEntity {
            entity_repo: self.entity_repo.clone(),
            id,
        }
    }
}
