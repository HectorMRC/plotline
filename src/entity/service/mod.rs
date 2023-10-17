mod create;
pub use create::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity};
use std::sync::Arc;

pub trait EntityRepository {
    fn create(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn create(&self, name: String) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            name,
            tags: Default::default(),
        }
    }

    pub fn remove(&self, names: Vec<String>) -> RemoveEntities<R> {
        RemoveEntities {
            entity_repo: self.entity_repo.clone(),
            names,
        }
    }
}
