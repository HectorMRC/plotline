use super::{ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{Error, ExperienceBuilder, ExperiencedEvent, Result},
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
            Err(Error::Collition)
        }
    }

    fn create(self) -> Result<()> {
        let _entity_tx = self.entity_repo.find(self.entity)?;
        let event_tx = self.event_repo.find(self.event)?;

        let experiences = self
            .experience_repo
            .filter(ExperienceFilter::default().with_entity(Some(self.entity)))?
            .into_iter()
            .map(Tx::begin)
            .collect::<Vec<_>>();

        let events = experiences
            .iter()
            .map(|experience| self.event_repo.find(experience.event).map_err(Into::into))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(Tx::begin)
            .collect::<Vec<_>>();

        if events.iter().any(|event| event.id() == self.event) {
            // Avoid DEADLOCK when acquiring the event_tx.
            return Err(Error::EventAlreadyExperienced);
        }

        let experienced_events = experiences
            .iter()
            .zip(events.iter())
            .map(|(experience, event)| ExperiencedEvent{
                _experience: experience,
                event
            })
            .collect::<Vec<_>>();

        let event = event_tx.begin();
        let mut select_closer = SelectCloserExperiences::new(self.event_repo.clone(), &event);

        experienced_events
            .iter()
            .map(|experienced_event| select_closer.with(&experienced_event))
            .collect::<Result<Vec<_>>>()?;

        // Actual logic
        let experience = ExperienceBuilder::new(self.event).build()?;
        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}

struct SelectCloserExperiences<'a, EventRepo>
where
    EventRepo: EventRepository,
{
    event_repo: Arc<EventRepo>,
    middle: &'a Event<EventRepo::Interval>,
    before: Option<ExperiencedEvent<'a, EventRepo::Interval>>,
    after: Option<ExperiencedEvent<'a, EventRepo::Interval>>,
}

impl<'a, EventRepo> SelectCloserExperiences<'a, EventRepo>
where
    EventRepo: EventRepository,
{
    fn new(event_repo: Arc<EventRepo>, middle: &'a Event<EventRepo::Interval>) -> Self {
        Self {
            event_repo,
            middle,
            before: None,
            after: None,
        }
    }

    fn with(&mut self, experience: &ExperiencedEvent<'a, EventRepo::Interval>) -> Result<()> {
        Ok(())
    }
}
