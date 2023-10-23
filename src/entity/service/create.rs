use std::{sync::Arc, marker::PhantomData};
use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, EntityID, Result, EntityName, error::Error},
    tag::Tags,
};

pub struct Uncomplete;
pub struct Complete;

pub struct CreateEntity<R, S = Uncomplete> {
    entity_repo: Arc<R>,
    state: PhantomData<S>,
    name: Option<EntityName>,
    id: Option<EntityID>,
    tags: Tags,
}

impl<R> CreateEntity<R, Complete>
where
    R: EntityRepository,
{
    pub fn execute(self) -> Result<Entity> {
        let Some(entity_name) = self.name else {
            return Err(Error::NotAnEntityName);
        };

        let mut entity = if let Some(entity_id) = self.id {
            Entity::with_id(entity_id, entity_name)
        } else {
            Entity::new(entity_name)
        };

        entity.tags = self.tags;

        self.entity_repo.create(&entity)?;
        Ok(entity)
    }
}

impl<R, S> CreateEntity<R, S> {
    pub fn with_name(self, name: EntityName) -> CreateEntity<R, Complete> {
        CreateEntity {
            entity_repo: self.entity_repo,
            state: PhantomData,
            name: Some(name),
            id: self.id,
            tags: self.tags,
        }
    }

    pub fn with_id(mut self, id: Option<EntityID>) -> Self {
        self.id = id;
        self
    }

    pub fn with_tags(mut self, tags: Tags) -> Self {
        self.tags = tags;
        self
    }
}

impl<R> EntityService<R>
where
    R: EntityRepository,
{
    pub fn create(&self) -> CreateEntity<R> {
        CreateEntity {
            entity_repo: self.entity_repo.clone(),
            state: PhantomData,
            name: Default::default(),
            id: Default::default(),
            tags: Default::default(),
        }
    }
}
