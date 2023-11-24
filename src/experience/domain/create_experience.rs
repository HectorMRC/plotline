use crate::{
    event::Event,
    experience::{Experience, ExperiencedEvent, Result},
};

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create_experience<Intv>(
    _event: &Event<Intv>,
    _experienced_events: &[ExperiencedEvent<'_, Intv>],
) -> Result<Experience<Intv>> {
    unimplemented!()
}
