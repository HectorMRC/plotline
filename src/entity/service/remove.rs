use super::EntityRepository;
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct RemoveEntities<R> {
    pub entity_repo: Arc<R>,
    pub names: Vec<String>,
}

impl<R> RemoveEntities<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Vec<Entity>> {
        Ok(vec![])
    }
}
