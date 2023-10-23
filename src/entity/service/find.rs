use super::{EntityFilter, EntityRepository, EntityService};
use crate::entity::{error::Result, Entity, Error};
use std::sync::Arc;

#[derive(Default)]
pub struct FindEntity<R> {
    entity_repo: Arc<R>,
    filter: EntityFilter,
}

impl<R> FindEntity<R>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Arc<Entity>> {
        let entities = self.entity_repo.filter(&self.filter)?;
        if entities.len() > 1 {
            return Err(Error::NotFound);
        }

        Ok(entities[0].clone())
    }
}

impl<R> FindEntity<R> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn find(&self) -> FindEntity<R> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
