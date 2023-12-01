use crate::{
    entity::Entity,
    experience::{Error, Experience, ExperienceBuilder, ExperiencedEvent, Result},
    id::Id,
    interval::Interval,
};
use std::collections::HashSet;
use super::{ConstraintGroup, constraint::Constraint, SelectCloserExperiences};

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<'a, Intv: Interval>(
    builder: ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) -> Result<Experience<Intv>> {
    let builder = builder.with_fallbacks(experienced_events)?;

    {
        let mut constaints_group = ConstraintGroup::with_defaults(&builder);
        experienced_events
            .iter()
            .try_for_each(|experienced_event| constaints_group.with(experienced_event))?;

        constaints_group.result()?;
    }

    builder.build()
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for any field set to [Option::None].
    fn with_fallbacks(mut self, experienced_events: &[ExperiencedEvent<'a, Intv>]) -> Result<Self> {
        if self.before.is_some() && self.after.is_some() {
            return Ok(self);
        }

        let (before, after) = SelectCloserExperiences::from_builder(&self)
            .with_iter(experienced_events.iter())
            .result();

        if self.after.is_none() {
            self.after = after
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

        let mut befores = before
            .map(|experienced_event| experienced_event.experience.after.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|profile| afters.contains(&profile.entity));

        self.before = befores.next();

        if let Some(other) = befores.next() {
            // there are multiple candidates to be choosen, one has to be specified
            let mut entities = vec![other.entity];
            if let Some(before) = self.before {
                entities.push(before.entity);
            }

            entities.extend(befores.map(|profile| profile.entity));
            return Err(Error::ExperienceMustBelongToOneOf(entities));
        }

        Ok(self)
    }
}