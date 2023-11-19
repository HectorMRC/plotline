use super::ExperienceRepository;
use crate::{
    entity::{service::EntityRepository, Entity},
    event::{service::EventRepository, Event},
    experience::{self, Error, Experience, Result},
    id::Id,
    profile::service::{ProfileFilter, ProfileRepository},
    transaction::{Tx, TxGuard},
};
use std::{
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex},
};

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, ProfileRepo, EntityRepo, EventRepo>
where
    EventRepo: EventRepository,
{
    experience_repo: Arc<ExperienceRepo>,
    profile_repo: Arc<ProfileRepo>,
    entity_repo: Arc<EntityRepo>,
    event_repo: Arc<EventRepo>,
    entity: Id<Entity>,
    event: Id<Event<EventRepo::Interval>>,
}

impl<ExperienceRepo, ProfileRepo, EntityRepo, EventRepo>
    SaveExperience<ExperienceRepo, ProfileRepo, EntityRepo, EventRepo>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    ProfileRepo: ProfileRepository,
    EntityRepo: EntityRepository,
    EventRepo: EventRepository,
{
    /// Executes the save experience transaction.
    pub fn execute(self) -> Result<()> {
        match self
            .experience_repo
            .find_by_entity_and_event(self.entity, self.event)
        {
            Ok(experience_tx) => self.update(experience_tx),
            Err(Error::NotFound) => self.create(),
            Err(err) => Err(err),
        }
    }

    fn create(self) -> Result<()> {
        let entity_tx = self.entity_repo.find(self.entity)?;
        let event_tx = self.event_repo.find(self.event)?;
        let profiles_tx = self
            .profile_repo
            .filter(&ProfileFilter::default().with_entity(Some(self.entity)))?;

        let experience = Experience::new(self.event);
        Ok(())
    }

    fn update(self, experience_tx: ExperienceRepo::Tx) -> Result<()> {
        Ok(())
    }
}
