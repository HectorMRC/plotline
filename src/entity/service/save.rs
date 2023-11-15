use super::{EntityRepository, EntityService};
use crate::{
    entity::{Entity, Result, Error},
    id::Id,
    name::Name, transaction::{Tx, TxGuard},
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
    pub fn execute(self) -> Result<Entity> {
        match self.entity_repo.find(self.id) {
            Ok(entity_tx) => self.update(entity_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err)
        }
    }

    fn create(self) -> Result<Entity>  {
        let entity = Entity::new(self.id, self.name);
        self.entity_repo.create(&entity)?;
        Ok(entity)
    }

    fn update(self, entity_tx: EntityRepo::Tx) -> Result<Entity>  {
        let mut entity = entity_tx.begin()?;
        entity.name = self.name;
        
        let data = entity.clone();
        entity.commit();

        Ok(data)
    }
}

impl<EntityRepo> EntityService<EntityRepo>
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
