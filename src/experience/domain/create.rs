use crate::{
    event::Event,
    experience::{Experience, ExperienceBuilder, ExperiencedEvent, Result},
    id::Identifiable,
    interval::Interval,
};
use std::cmp;

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<Intv: Interval>(
    event: &Event<Intv>,
    experienced_events: &[ExperiencedEvent<'_, Intv>],
) -> Result<Experience<Intv>> {
    let mut select_closer = SelectCloserExperiences::new(event);
    experienced_events.iter().for_each(|experienced_event| {
        select_closer.with(experienced_event);
    });

    ExperienceBuilder::new(event.id())
        .with_before(
            select_closer
                .before
                .and_then(|experienced_event| experienced_event.experience.after.clone()),
        )
        .with_after(
            select_closer
                .after
                .and_then(|experienced_event| experienced_event.experience.before.clone()),
        )
        .build()
}

struct SelectCloserExperiences<'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<&'a ExperiencedEvent<'a, Intv>>,
    after: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> SelectCloserExperiences<'a, Intv>
where
    Intv: Interval,
{
    fn new(event: &'a Event<Intv>) -> Self {
        SelectCloserExperiences {
            event,
            before: None,
            after: None,
        }
    }

    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) {
        if experienced_event.event.hi() < self.event.lo() {
            self.before = cmp::max(self.before, Some(experienced_event));
        } else if experienced_event.event.lo() > self.event.hi() {
            self.after = cmp::min(self.after, Some(experienced_event));
        }
    }
}
