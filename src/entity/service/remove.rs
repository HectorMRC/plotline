use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct RemoveEntities<R> {
    entity_repo: Arc<R>,
    names: Vec<String>,
}

impl<R> RemoveEntities<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Vec<Entity>> {
        Ok(vec![])
    }
}

impl<R> RemoveEntities<R> {
    pub fn with_names(mut self, names: Vec<String>) -> Self {
        self.names = names;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn remove(&self) -> RemoveEntities<R> {
        RemoveEntities {
            entity_repo: self.entity_repo.clone(),
            names: Default::default(),
        }
    }
}
