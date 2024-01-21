use super::{ConstraintFactory, ExperienceApplication, ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{
        constraint::Constraint, Error, ExperienceBuilder, ExperiencedEvent, Profile, Result,
    },
    id::{Id, Identifiable},
    transaction::Tx,
};
use std::{marker::PhantomData, sync::Arc};

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    EventRepo: EventRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    entity_repo: Arc<EntityRepo>,
    event_repo: Arc<EventRepo>,
    cnst_factory: PhantomData<CnstFactory>,
    entity: Id<Entity>,
    event: Id<Event<EventRepo::Interval>>,
    after: Option<Vec<Profile>>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    EventRepo: EventRepository,
{
    pub fn with_after(mut self, after: Option<Vec<Profile>>) -> Self {
        self.after = after;
        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
    CnstFactory: ConstraintFactory<EventRepo::Interval>,
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

        let experienced_events = experiences
            .iter()
            .zip(events.iter())
            .map(|(experience, event)| ExperiencedEvent { experience, event })
            .collect::<Vec<_>>();

        let experience = ExperienceBuilder::new(&entity, &event)
            .with_after(self.after)
            .with_fallbacks(&experienced_events)
            .build()?;

        let experienced_event = ExperiencedEvent {
            experience: &experience,
            event: &event,
        };

        experienced_events
            .iter()
            .try_fold(
                CnstFactory::new(&experienced_event),
                |constraint, experienced_event| constraint.with(experienced_event),
            )?
            .result()?;

        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
    CnstFactory: ConstraintFactory<EventRepo::Interval>,
{
    pub fn save_experience(
        &self,
        entity_id: Id<Entity>,
        event_id: Id<Event<EventRepo::Interval>>,
    ) -> SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory> {
        SaveExperience {
            experience_repo: self.experience_repo.clone(),
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
            cnst_factory: PhantomData,
            entity: entity_id,
            event: event_id,
            after: Default::default(),
        }
    }
}
