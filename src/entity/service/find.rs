use super::{EntityFilter, EntityRepository, EntityService};
use crate::{
    entity::{error::Result, Entity, Error},
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must be retrived.
#[derive(Default)]
pub struct FindEntity<R> {
    entity_repo: Arc<R>,
    filter: EntityFilter,
}

impl<R> FindEntity<R>
where
    R: EntityRepository,
{
    /// Executes the find query, through which one, and exactly one, entity must be retrived.
    /// If there is no entity matching the query the error [Error::NotFound] is returned.
    pub fn execute(self) -> Result<Entity> {
        let entities = self.entity_repo.filter(&self.filter)?;
        if entities.len() > 1 {
            return Err(Error::NotFound);
        }

        let entity = entities[0].begin()?;
        Ok(entity.clone())
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
    pub fn find_entity(&self) -> FindEntity<R> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
