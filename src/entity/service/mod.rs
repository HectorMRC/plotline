mod create;
pub use create::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity, EntityID};
use std::sync::Arc;

pub trait EntityRepository {
    fn find(&self, id: &EntityID) -> Result<Arc<Entity>>;
    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Arc<Entity>>>;
    fn create(&self, entity: &Entity) -> Result<()>;
    fn remove(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}
