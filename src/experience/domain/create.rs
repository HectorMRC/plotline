use super::{Constraint, LiFoConstraintChain, SelectNextExperience, SelectPreviousExperience};
use crate::{
    experience::{Experience, ExperienceBuilder, ExperiencedEvent, Result},
    id::Identifiable,
    interval::Interval,
};

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<'a, Intv: Interval>(
    builder: ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) -> Result<Experience<Intv>> {
    let event = builder.event;
    let experience = builder.with_fallbacks(experienced_events).build()?;
    let experienced_event = ExperiencedEvent {
        experience: &experience,
        event,
    };

    let constraint = LiFoConstraintChain::with_defaults(&experienced_event);
    let constraint = experienced_events
        .iter()
        .try_fold(constraint, |constraint, experienced_event| {
            constraint.with(experienced_event)
        })?;

    constraint.result()?;
    Ok(experience)
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for those fields set to [Option::None].
    fn with_fallbacks(mut self, experienced_events: &[ExperiencedEvent<'a, Intv>]) -> Self {
        let mut previous = SelectPreviousExperience::new(self.event);
        let mut next = SelectNextExperience::new(self.event);
        for experienced_event in experienced_events.iter() {
            previous = previous.with(experienced_event);
            next = next.with(experienced_event);
        }

        self.after = self.after.or(previous
            .value()
            .or(next.value())
            .and_then(|experienced_event| {
                experienced_event
                    .experience
                    .after
                    .iter()
                    .find(|profile| profile.entity == self.entity.id())
                    .cloned()
            })
            .map(|profile| vec![profile]));

        self
    }
}
