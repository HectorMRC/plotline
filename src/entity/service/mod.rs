mod create;
pub use create::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity, EntityId};
use crate::id::Id;
use std::sync::Arc;

pub trait EntityRepository {
    fn find(&self, id: &Id<EntityId>) -> Result<Arc<Entity>>;
    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Arc<Entity>>>;
    fn create(&self, entity: &Entity) -> Result<()>;
    fn delete(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}
