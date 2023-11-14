use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity};
use crate::transaction::Tx;
use crate::{id::Id, name::Name};
use std::sync::Arc;

macro_rules! equals_or_return {
    ($option:expr, $subject:expr) => {
        if $option
            .as_ref()
            .map(|want| want != $subject)
            .unwrap_or_default()
        {
            return false;
        }
    };
}

/// Implements the filter query, through which zero o more entities may be retrived.
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
pub struct FilterEntities<R> {
    entity_repo: Arc<R>,
    filter: EntityFilter,
}

impl<R> FilterEntities<R>
where
    R: EntityRepository,
{
    /// Executes the filter query, through which zero o more entities may be retrived.
    pub fn execute(self) -> Result<Vec<Entity>> {
        let entities_tx = self.entity_repo.filter(&self.filter)?;
        let mut entities = Vec::with_capacity(entities_tx.len());
        for entity_tx in entities_tx {
            let entity = entity_tx.begin()?;
            entities.push(entity.as_ref().clone());
        }

        Ok(entities)
    }
}

impl<R> FilterEntities<R> {
    pub fn with_filter(mut self, filter: EntityFilter) -> Self {
        self.filter = filter;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn filter_entities(&self) -> FilterEntities<R> {
        FilterEntities {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
