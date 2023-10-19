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
