use super::{EntityRepository, EntityService};
use crate::entity::{error::Result, Entity, EntityID, EntityName};
use crate::{id::ID, name::Name};
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

#[derive(Default)]
pub struct EntityFilter {
    name: Option<Name<EntityName>>,
    id: Option<ID<EntityID>>,
}

impl EntityFilter {
    pub fn with_id(mut self, id: Option<ID<EntityID>>) -> Self {
        self.id = id;
        self
    }

    pub fn with_name(mut self, name: Option<Name<EntityName>>) -> Self {
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
    pub fn execute(self) -> Result<Vec<Arc<Entity>>> {
        self.entity_repo.filter(&self.filter)
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
    pub fn filter(&self) -> FilterEntities<R> {
        FilterEntities {
            entity_repo: self.entity_repo.clone(),
            filter: Default::default(),
        }
    }
}
