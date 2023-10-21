mod create;
pub use create::*;

mod list;
pub use list::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity, EntityName};
use std::sync::Arc;

pub trait EntityRepository {
    fn find_by_name(&self, name: &EntityName) -> Result<Arc<Entity>>;
    fn list(&self) -> Result<Vec<Arc<Entity>>>;
    fn create(&self, entity: &Entity) -> Result<()>;
    fn remove(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}
