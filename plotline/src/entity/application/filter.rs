use futures::future;

use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{Entity, Result},
    id::Id,
    name::Name,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the filter query, through which zero o more entities may be
/// retrived.
#[derive(Default)]
pub struct EntityFilter {
    pub name: Option<Name<Entity>>,
    pub id: Option<Id<Entity>>,
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
    pub async fn execute(self) -> Result<Vec<Entity>> {
        Ok(future::join_all(
            self.entity_repo
                .filter(&self.filter)
                .await?
                .into_iter()
                .map(|entity_tx| async move { entity_tx.read().await.clone() }),
        )
        .await)
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
