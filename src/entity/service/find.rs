use super::{EntityFilter, EntityRepository, EntityService};
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct FindEntity<R> {
    entity_repo: Arc<R>,
    filter: EntityFilter,
}

impl<R> FindEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Arc<Entity>> {
        self.entity_repo.find(&self.filter)
    }
}

impl<R> FindEntity<R> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<R> EntityService<R> {
    pub fn find(&self) -> FindEntity<R> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
