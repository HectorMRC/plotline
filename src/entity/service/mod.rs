mod create;
pub use create::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity};
use crate::id::Id;
use std::sync::Arc;

pub trait EntityRepository {
    fn find(&self, id: &Id<Entity>) -> Result<Entity>;
    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Entity>>;
    fn create(&self, entity: &Entity) -> Result<()>;
    fn delete(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}
