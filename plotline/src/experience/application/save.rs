use super::{ConstraintFactory, ExperienceApplication, ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{
        constraint::Constraint, Error, Experience, ExperienceBuilder, ExperiencedEvent, Profile,
        Result,
    },
    id::Id,
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
    id: Id<Experience<EventRepo::Interval>>,
    entity: Option<Id<Entity>>,
    event: Option<Id<Event<EventRepo::Interval>>>,
    profiles: Option<Vec<Profile>>,
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    EventRepo: EventRepository,
{
    pub fn with_profiles(mut self, profiles: Option<Vec<Profile>>) -> Self {
        self.profiles = profiles;
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
    pub fn with_entity(mut self, entity: Option<Id<Entity>>) -> Self {
        self.entity = entity;
        self
    }

    pub fn with_event(mut self, event: Option<Id<Event<EventRepo::Interval>>>) -> Self {
        self.event = event;
        self
    }

    /// Executes the save experience transaction.
    pub fn execute(self) -> Result<()> {
        match self.experience_repo.find(self.id) {
            Ok(experience_tx) => self.update(experience_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let entity_tx = self
            .entity_repo
            .find(self.entity.ok_or(Error::MandatoryField("entity"))?)?;

        let entity = entity_tx.read();

        let event_tx = self
            .event_repo
            .find(self.event.ok_or(Error::MandatoryField("event"))?)?;

        // read-only ensures the same event could be locked for reading more
        // than once.
        let event = event_tx.read();

        let experiences = self
            .experience_repo
            .filter(&ExperienceFilter::default().with_entity(self.entity))?
            .into_iter()
            .map(Tx::read)
            .collect::<Vec<_>>();

        let events = experiences
            .iter()
            .map(|experience| self.event_repo.find(experience.event).map_err(Into::into))
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            // read-only ensure no dead-lock happens for the same event.
            .map(Tx::read)
            .collect::<Vec<_>>();

        let experienced_events = experiences
            .iter()
            .zip(events.iter())
            .map(|(experience, event)| ExperiencedEvent { experience, event })
            .collect::<Vec<_>>();

        let experience = ExperienceBuilder::new(self.id, &entity, &event)
            .with_profiles(self.profiles)
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
        id: Id<Experience<EventRepo::Interval>>,
    ) -> SaveExperience<ExperienceRepo, EntityRepo, EventRepo, CnstFactory> {
        SaveExperience {
            experience_repo: self.experience_repo.clone(),
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
            cnst_factory: PhantomData,
            id,
            entity: Default::default(),
            event: Default::default(),
            profiles: Default::default(),
        }
    }
}
