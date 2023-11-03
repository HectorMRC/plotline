use super::{EntityRepository, EntityService};
use crate::{
    entity::{error::Result, Entity},
    id::Id,
};
use std::sync::Arc;

pub struct RemoveEntity<R> {
    entity_repo: Arc<R>,
    id: Id<Entity>,
}

impl<R> RemoveEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Arc<Entity>> {
        let entity = self.entity_repo.find(&self.id)?;
        self.entity_repo.delete(&entity).map(|_| entity)
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
