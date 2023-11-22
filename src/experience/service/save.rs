use super::{ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{service::EntityRepository, Entity},
    event::{service::EventRepository, Event},
    experience::{Error, Experience, Result},
    id::{Id, Identifiable},
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, EntityRepo, EventRepo>
where
    EventRepo: EventRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    entity_repo: Arc<EntityRepo>,
    event_repo: Arc<EventRepo>,
    entity: Id<Entity>,
    event: Id<Event<EventRepo::Interval>>,
}

impl<ExperienceRepo, EntityRepo, EventRepo> SaveExperience<ExperienceRepo, EntityRepo, EventRepo>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
{
    /// Executes the save experience transaction.
    pub fn execute(self) -> Result<()> {
        let mut experiences_tx = self.experience_repo.filter(
            ExperienceFilter::default()
                .with_event(Some(self.event))
                .with_entity(Some(self.entity)),
        )?;

        if experiences_tx.is_empty() {
            self.create()
        } else if experiences_tx.len() == 1 {
            self.update(experiences_tx.remove(0))
        } else {
            Err(Error::Collition {
                entity: self.entity.to_string(),
                event: self.event.to_string(),
            })
        }
    }

    fn create(self) -> Result<()> {
        // Acquire resources
        let entity_tx = self.entity_repo.find(self.entity)?;
        let entity = entity_tx.begin()?;

        let event_tx = self.event_repo.find(self.event)?;
        let event = event_tx.begin()?;

        let _experiences_tx = self
            .experience_repo
            .filter(ExperienceFilter::default().with_entity(Some(entity.id())))?;

        // Actual logic
        let experience = Experience::new(event.id());
        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}
