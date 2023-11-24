//! The domain service.

use super::{Experience, ExperiencedEvent, Result};
use crate::event::Event;

pub struct ExperienceService;

impl ExperienceService {
    /// Creates a new experience caused by the given event as long as it fits
    /// in the given ordered succession of experienced events.
    pub fn create_experience<Intv>(
        _event: &Event<Intv>,
        _experienced_events: &[ExperiencedEvent<'_, Intv>],
    ) -> Result<Experience<Intv>> {
        unimplemented!()
    }
}
