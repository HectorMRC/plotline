use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity, EntityName};
use std::sync::Arc;

pub struct EntityRemoveByName<R> {
    entity_repo: Arc<R>,
    name: String,
}

impl<R> EntityRemoveByName<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Arc<Entity>> {
        let entity_name: EntityName = self.name.try_into()?;
        let entity = self.entity_repo.find_by_name(&entity_name)?;
        self.entity_repo.remove(entity.as_ref()).map(|_| entity)
    }
}

impl<R> EntityRemoveByName<R> {
    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn remove_by_name(&self) -> EntityRemoveByName<R> {
        EntityRemoveByName {
            entity_repo: self.entity_repo.clone(),
            name: Default::default(),
        }
    }
}
