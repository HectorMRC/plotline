use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct EntityList<R> {
    entity_repo: Arc<R>,
}

impl<R> EntityList<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Vec<Arc<Entity>>> {
        self.entity_repo.list()
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn list(&self) -> EntityList<R> {
        EntityList {
            entity_repo: self.entity_repo.clone(),
        }
    }
}
