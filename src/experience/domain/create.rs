use super::{
    constraint::Constraint, ConstraintGroup, SelectNextExperience, SelectPreviousExperience,
};
use crate::{
    entity::Entity,
    experience::{Experience, ExperienceBuilder, ExperiencedEvent, Result},
    id::Id,
    interval::Interval,
};
use std::collections::HashSet;

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<'a, Intv: Interval>(
    builder: ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) -> Result<Experience<Intv>> {
    let builder = builder.with_fallbacks(experienced_events);
    let mut constaints_group = ConstraintGroup::with_defaults(&builder);
    experienced_events
        .iter()
        .try_for_each(|experienced_event| constaints_group.with(experienced_event))?;

    constaints_group.result()?;
    builder.build()
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for any field set to [Option::None].
    fn with_fallbacks(mut self, experienced_events: &[ExperiencedEvent<'a, Intv>]) -> Self {
        if self.before.is_some() && self.after.is_some() {
            return self;
        }

        let mut previous = SelectPreviousExperience::from_builder(&self);
        let mut next = SelectNextExperience::from_builder(&self);
        for experienced_event in experienced_events.iter() {
            previous = previous.with(experienced_event);
            next = next.with(experienced_event);
        }

        let previous = previous.value();
        let next = next.value();

        if self.after.is_none() {
            self.after = next
                .and_then(|experienced_event| experienced_event.experience.before.clone())
                .map(|before| vec![before]);
        }

        let afters: HashSet<Id<Entity>> = self
            .after
            .as_ref()
            .map(|experienced_events| {
                HashSet::from_iter(experienced_events.iter().map(|profile| profile.entity))
            })
            .unwrap_or_default();

        let mut befores = previous
            .map(|experienced_event| experienced_event.experience.after.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|profile| afters.contains(&profile.entity));

        let before = befores.next();
        if befores.next().is_none() {
            self.before = before;
        }

        self
    }
}
