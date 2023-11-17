use super::{EventRepository, EventService};
use crate::{
    entity::{service::EntityRepository, Entity},
    event::{Event, Result},
    id::Id,
    transaction::{Tx, TxGuard},
};
use std::sync::Arc;

/// Implements the add entity transaction for an event.
pub struct AddEntity<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    entity_repo: Arc<EntityRepo>,
    entity_id: Id<Entity>,
    event_id: Id<Event<EventRepo::Interval>>,
}

impl<EventRepo, EntityRepo> AddEntity<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
    EntityRepo: EntityRepository,
{
    /// Executes the add entity transation.
    pub fn execute(self) -> Result<()> {
        self.entity_repo.find(self.entity_id)?;
        let event_tx = self.event_repo.find(self.event_id)?;
        let mut event = event_tx.begin()?;

        event.entities.push(self.entity_id);

        event.commit();
        Ok(())
    }
}

impl<EventRepo, EntityRepo> EventService<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
    EntityRepo: EntityRepository,
{
    pub fn add_entity(
        &self,
        entity_id: Id<Entity>,
        event_id: Id<Event<EventRepo::Interval>>,
    ) -> AddEntity<EventRepo, EntityRepo> {
        AddEntity {
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
            entity_id,
            event_id,
        }
    }
}
