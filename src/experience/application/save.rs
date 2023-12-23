use super::{ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{domain, Error, ExperienceBuilder, ExperiencedEvent, Result},
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
        let experiences_tx = self.experience_repo.filter(
            ExperienceFilter::default()
                .with_event(Some(self.event))
                .with_entity(Some(self.entity)),
        )?;

        // there should be impossible to have more than one
        if let Some(experience) = experiences_tx.into_iter().next() {
            self.update(experience)
        } else {
            self.create()
        }
    }

    fn create(self) -> Result<()> {
        let entity_tx = self.entity_repo.find(self.entity)?;
        let entity = entity_tx.begin();

        let event_tx = self.event_repo.find(self.event)?;
        let event = event_tx.begin();

        let experiences = self
            .experience_repo
            .filter(ExperienceFilter::default().with_entity(Some(entity.id())))?
            .into_iter()
            .map(Tx::begin)
            .collect::<Vec<_>>();

        if experiences
            .iter()
            .any(|experience| experience.event == event.id())
        {
            // Avoid deadlock by acquiring the same event twice.
            return Err(Error::EventAlreadyExperienced);
        }

        let events = experiences
            .iter()
            .map(|experience| self.event_repo.find(experience.event).map_err(Into::into))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(Tx::begin)
            .collect::<Vec<_>>();

        let mut experienced_events = experiences
            .iter()
            .zip(events.iter())
            .map(|(experience, event)| ExperiencedEvent { experience, event })
            .collect::<Vec<_>>();

        experienced_events.sort_by(|a, b| a.event.cmp(b.event));
        let experience =
            domain::create(ExperienceBuilder::new(&entity, &event), &experienced_events)?;
        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}
