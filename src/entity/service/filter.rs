use super::{EntityFilter, EntityRepository, EntityService};
use crate::entity::{error::Result, Entity};
use std::sync::Arc;

pub struct FilterEntities<R> {
    _entity_repo: Arc<R>,
    filter: EntityFilter,
}

impl<R> FilterEntities<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Vec<Arc<Entity>>> {
        todo!()
    }
}

impl<R> FilterEntities<R> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<R> EntityService<R> {
    pub fn filter(&self) -> FilterEntities<R> {
        FilterEntities {
            _entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
