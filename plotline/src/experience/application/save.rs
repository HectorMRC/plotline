use super::{ExperienceApplication, ExperienceFilter, ExperienceRepository};
use crate::{
    entity::{application::EntityRepository, Entity},
    event::{application::EventRepository, Event},
    experience::{Error, Experience, ExperienceBuilder, Profile, Result},
    id::Id,
    transaction::Tx,
};
use std::{ops::Deref, sync::Arc};

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, EntityRepo, EventRepo>
where
    EventRepo: EventRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    entity_repo: Arc<EntityRepo>,
    event_repo: Arc<EventRepo>,
    id: Id<Experience<EventRepo::Interval>>,
    entity: Option<Id<Entity>>,
    event: Option<Id<Event<EventRepo::Interval>>>,
    profiles: Option<Vec<Profile>>,
}

impl<ExperienceRepo, EntityRepo, EventRepo> SaveExperience<ExperienceRepo, EntityRepo, EventRepo>
where
    EventRepo: EventRepository,
{
    pub fn with_profiles(mut self, profiles: Option<Vec<Profile>>) -> Self {
        self.profiles = profiles;
        self
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo> SaveExperience<ExperienceRepo, EntityRepo, EventRepo>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
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
        let entity = self
            .entity_repo
            .find(self.entity.ok_or(Error::MandatoryField("entity"))?)?
            .read();

        let event = self
            .event_repo
            .find(self.event.ok_or(Error::MandatoryField("event"))?)?
            .read();

        let experiences = self
            .experience_repo
            .filter(&ExperienceFilter::default().with_entity(self.entity))?
            .into_iter()
            .map(Tx::read)
            .collect::<Vec<_>>();

        let experience = ExperienceBuilder::new(self.id, entity.clone(), event.clone())
            .with_profiles(self.profiles)
            .with_fallbacks(experiences.iter().map(|experience| experience.deref()))
            .build()?;

        self.experience_repo.create(&experience)?;
        Ok(())
    }

    fn update(self, _experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
{
    pub fn save_experience(
        &self,
        id: Id<Experience<EventRepo::Interval>>,
    ) -> SaveExperience<ExperienceRepo, EntityRepo, EventRepo> {
        SaveExperience {
            experience_repo: self.experience_repo.clone(),
            entity_repo: self.entity_repo.clone(),
            event_repo: self.event_repo.clone(),
            id,
            entity: Default::default(),
            event: Default::default(),
            profiles: Default::default(),
        }
    }
}
