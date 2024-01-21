use super::{EntityApplication, EntityRepository};
use crate::{
    entity::{Entity, Error, Result},
    id::Id,
    name::Name,
    transaction::{Tx, TxGuard},
};
use std::sync::Arc;

/// Implements the save entity transaction.
pub struct SaveEntity<EntityRepo> {
    entity_repo: Arc<EntityRepo>,
    name: Name<Entity>,
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
        let entity = Entity::new(self.id, self.name);
        self.entity_repo.create(&entity)
    }

    fn update(self, entity_tx: EntityRepo::Tx) -> Result<()> {
        let mut entity = entity_tx.begin();
        entity.name = self.name;

        entity.commit();
        Ok(())
    }
}

impl<EntityRepo> EntityApplication<EntityRepo>
where
    EntityRepo: EntityRepository,
{
    pub fn save_entity(&self, id: Id<Entity>, name: Name<Entity>) -> SaveEntity<EntityRepo> {
        SaveEntity {
            entity_repo: self.entity_repo.clone(),
            name,
            id,
        }
    }
}
