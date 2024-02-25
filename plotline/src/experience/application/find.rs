use super::{ExperienceApplication, ExperienceRepository};
use crate::{
    event::application::EventRepository,
    experience::{Experience, Result},
    id::Identifiable,
    interval::Interval,
    transaction::Tx,
};
use std::sync::Arc;

/// Implements the find query, through which one, and exactly one, entity must
/// be retrived.
#[derive(Default)]
pub struct FindExperience<ExperienceRepo, Intv> {
    experience_repo: Arc<ExperienceRepo>,
    id: <Experience<Intv> as Identifiable>::Id,
}

impl<ExperienceRepo, Intv> FindExperience<ExperienceRepo, Intv>
where
    ExperienceRepo: ExperienceRepository<Interval = Intv>,
    Intv: Interval,
{
    /// Executes the find query, through which one, and exactly one, experience
    /// must be retrived.
    pub fn execute(self) -> Result<Experience<Intv>> {
        Ok(self.experience_repo.find(self.id)?.read().clone())
    }
}

impl<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
    ExperienceApplication<ExperienceRepo, EntityRepo, EventRepo, CnstFactory>
where
    ExperienceRepo: ExperienceRepository<Interval = EventRepo::Interval>,
    EventRepo: EventRepository,
{
    pub fn find_experience(
        &self,
        id: <Experience<EventRepo::Interval> as Identifiable>::Id,
    ) -> FindExperience<ExperienceRepo, EventRepo::Interval> {
        FindExperience {
            experience_repo: self.experience_repo.clone(),
            id,
        }
    }
}
