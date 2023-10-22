mod create;
pub use create::*;

mod filter;
pub use filter::*;

mod find;
pub use find::*;

mod remove;
pub use remove::*;

use super::{error::Result, Entity, EntityID, EntityName};
use std::sync::Arc;

macro_rules! assert_or_return {
    ($option:expr, $subject:expr) => {
        if $option
            .as_ref()
            .map(|want| want != &$subject)
            .unwrap_or_default()
        {
            return false;
        }
    };
}

#[derive(Default)]
pub struct EntityFilter {
    name: Option<EntityName>,
    id: Option<EntityID>,
}

impl EntityFilter {
    pub fn with_id(mut self, id: Option<EntityID>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: Option<EntityName>) -> Self {
        self.name = name;
        self
    }

    pub fn filter(&self, entity: &Entity) -> bool {
        assert_or_return!(self.name, entity.name);
        assert_or_return!(self.id, entity.id);
        true
    }
}

pub trait EntityRepository {
    fn find(&self, filter: &EntityFilter) -> Result<Arc<Entity>>;
    fn filter(&self, filter: &EntityFilter) -> Result<Vec<Arc<Entity>>>;
    fn create(&self, entity: &Entity) -> Result<()>;
    fn remove(&self, entity: &Entity) -> Result<()>;
}

pub struct EntityService<R> {
    pub entity_repo: Arc<R>,
}
