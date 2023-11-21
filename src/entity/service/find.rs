use super::{EntityFilter, EntityRepository, EntityService};
use crate::{
    entity::{error::Result, Entity, Error},
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    filter: EntityFilter,
}

impl<EntityRepo> FindEntity<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the find query, through which one, and exactly one, entity must
    /// be retrived. If there is no entity matching the query the error
    /// [Error::NotFound] is returned.
    pub fn execute(self) -> Result<Entity> {
        let entities = self.entity_repo.filter(&self.filter)?;
        if entities.len() > 1 {
            return Err(Error::NotFound);
        }

        let entity = entities[0].begin()?;
        Ok(entity.clone())
    }
}

impl<EntityRepo> FindEntity<EntityRepo> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<EntityRepo> EntityService<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn find_entity(&self) -> FindEntity<EntityRepo> {
        FindEntity {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
