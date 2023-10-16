use super::EntityRepository;
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct CreateEntity<R> {
    pub entity_repo: Arc<R>,
    pub name: String,
}

impl<R> CreateEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let entity = self.name.try_into().map(Entity::new).unwrap();
        self.entity_repo.create(&entity).unwrap();
        Ok(entity)
    }
}
