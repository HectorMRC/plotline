use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{Entity, Error, Result},
    id::Id,
    name::{Error as NameError, Name},
    transaction::{Tx, TxWriteGuard},
    update_if_some,
};
use std::sync::Arc;

/// Implements the save entity transaction.
pub struct SaveEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    name: Option<Name<Entity>>,
    id: Id<Entity>,
}

impl<EntityRepo> SaveEntity<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    /// Executes the save entity transaction.
    pub fn execute(self) -> Result<()> {
        match self.entity_repo.find(self.id) {
            Ok(entity_tx) => self.update(entity_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let entity = Entity::new(self.id, self.name.ok_or(NameError::NotAName)?);
        self.entity_repo.create(&entity)
    }

    fn update(self, entity_tx: EntityRepo::Tx) -> Result<()> {
        let mut entity = entity_tx.write();

        update_if_some(&mut entity.name, self.name);

        entity.commit();
        Ok(())
    }
}

impl<EntityRepo> SaveEntity<EntityRepo> {
    pub fn with_name(mut self, name: Option<Name<Entity>>) -> Self {
        self.name = name;
        self
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn save_entity(&self, id: Id<Entity>) -> SaveEntity<EntityRepo> {
        SaveEntity {
            entity_repo: self.entity_repo.clone(),
            name: Default::default(),
            id,
        }
    }
}
