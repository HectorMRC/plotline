//! The domain service.

use super::{Result, Experience, ExperienceBuilder, ExperiencedEvent};

pub struct ExperienceService;

impl ExperienceService {
    /// Creates a new experience as long as no constraints are violated by doing so.
    pub fn create_experience<Intv>(builder: ExperienceBuilder<Intv>, _experienced_events: &[ExperiencedEvent<'_, Intv>]) -> Result<Experience<Intv>> {
        builder.build()
    }
}