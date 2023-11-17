use super::{EventRepository, EventService};
use crate::{
    entity::{service::EntityRepository, Entity},
    event::Event,
    event::{Error, Result},
    id::Id,
    name::Name,
    transaction::{Tx, TxGuard},
};
use std::sync::Arc;

/// Implements the save event transaction.
pub struct SaveEvent<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
    EntityRepo: EntityRepository,
{
    event_repo: Arc<EventRepo>,
    entity_repo: Arc<EntityRepo>,
    id: Id<Event<EventRepo::Interval>>,
    name: Option<Name<Event<EventRepo::Interval>>>,
    interval: Option<EventRepo::Interval>,
    entities: Option<Vec<Id<Entity>>>,
}

impl<EventRepo, EntityRepo> SaveEvent<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
    EntityRepo: EntityRepository,
{
    pub fn with_name(mut self, name: Option<Name<Event<EventRepo::Interval>>>) -> Self {
        self.name = name;
        self
    }

    pub fn with_interval(mut self, intv: Option<EventRepo::Interval>) -> Self {
        self.interval = intv;
        self
    }

    pub fn with_entities(mut self, entities: Option<Vec<Id<Entity>>>) -> Self {
        self.entities = entities;
        self
    }

    /// Executes the create event transaction.
    pub fn execute(self) -> Result<()> {
        if let Some(entities) = &self.entities {
            for entity_id in entities {
                self.entity_repo.find(*entity_id)?;
            }
        }

        match self.event_repo.find(self.id) {
            Ok(event_tx) => self.update(event_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let Some(name) = self.name else {
            return Err(Error::NameRequired);
        };

        let Some(interval) = self.interval else {
            return Err(Error::IntervalRequired);
        };

        let event =
            Event::new(self.id, name, interval).with_entities(self.entities.unwrap_or_default());

        self.event_repo.create(&event)
    }

    fn update(self, event_tx: EventRepo::Tx) -> Result<()> {
        let mut event = event_tx.begin()?;

        if let Some(name) = self.name {
            event.name = name;
        }

        if let Some(interval) = self.interval {
            event.interval = interval;
        }

        if let Some(entities) = self.entities {
            event.entities = entities;
        }

        event.commit();
        Ok(())
    }
}

impl<EventRepo, EntityRepo> EventService<EventRepo, EntityRepo>
where
    EventRepo: EventRepository,
    EntityRepo: EntityRepository,
{
    pub fn save_event(
        &self,
        id: Id<Event<EventRepo::Interval>>,
    ) -> SaveEvent<EventRepo, EntityRepo> {
        SaveEvent {
            event_repo: self.event_repo.clone(),
            entity_repo: self.entity_repo.clone(),
            id,
            name: Default::default(),
            interval: Default::default(),
            entities: Default::default(),
        }
    }
}
