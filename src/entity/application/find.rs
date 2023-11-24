use super::{EntityFilter, EntityRepository, EntityApplication};
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
    /// be retrived.
    pub fn execute(self) -> Result<Entity> {
        let mut entities = self.entity_repo.filter(&self.filter)?;
        if entities.is_empty() {
            return Err(Error::NotFound);
        }

        if entities.len() > 1 {
            return Err(Error::MoreThanOne);
        }

        Ok(entities.remove(0).begin().clone())
    }
}

impl<EntityRepo> FindEntity<EntityRepo> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
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
