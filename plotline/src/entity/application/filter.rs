use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{Entity, Result},
    id::Id,
    macros::equals_or_return,
    name::Name,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the filter query, through which zero o more entities may be
/// retrived.
#[derive(Default)]
pub struct EntityFilter {
    name: Option<Name<Entity>>,
    id: Option<Id<Entity>>,
}

impl EntityFilter {
    pub fn with_id(mut self, id: Option<Id<Entity>>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: Option<Name<Entity>>) -> Self {
        self.name = name;
        self
    }

    pub fn filter(&self, entity: &Entity) -> bool {
        equals_or_return!(self.name, &entity.name);
        equals_or_return!(self.id, &entity.id);
        true
    }
}

#[derive(Default)]
pub struct FilterEntities<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    filter: EntityFilter,
}

impl<EntityRepo> FilterEntities<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the filter query, through which zero o more entities may be
    /// retrived.
    pub fn execute(self) -> Result<Vec<Entity>> {
        Ok(self
            .entity_repo
            .filter(&self.filter)?
            .into_iter()
            .map(Tx::begin)
            .map(|entity| entity.clone())
            .collect())
    }
}

impl<EntityRepo> FilterEntities<EntityRepo> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn filter_entities(&self) -> FilterEntities<EntityRepo> {
        FilterEntities {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
