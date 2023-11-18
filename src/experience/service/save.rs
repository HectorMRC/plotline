use super::ExperienceRepository;
use crate::{
    event::service::EventRepository, experience::Result, profile::service::ProfileRepository,
};
use std::sync::Arc;

/// Implements the save experience transaction.
pub struct SaveExperience<ExperienceRepo, ProfileRepo, EventRepo> {
    experience_repo: Arc<ExperienceRepo>,
    profile_repo: Arc<ProfileRepo>,
    event_repo: Arc<EventRepo>,
}

impl<ExperienceRepo, ProfileRepo, EventRepo> SaveExperience<ExperienceRepo, ProfileRepo, EventRepo>
where
    ExperienceRepo: ExperienceRepository,
    ProfileRepo: ProfileRepository,
    EventRepo: EventRepository,
{
    /// Executes the save experience transaction.
    pub fn execute(self) -> Result<()> {
        Ok(())
    }
}
