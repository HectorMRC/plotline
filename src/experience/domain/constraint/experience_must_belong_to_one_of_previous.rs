use super::Constraint;
use crate::{
    entity::Entity,
    experience::{
        domain::SelectPreviousExperience, Error, ExperienceBuilder, ExperiencedEvent, Result,
    },
    id::Id,
    interval::Interval,
};
use std::collections::HashSet;

pub struct ExperienceMustBelongToOneOfPrevious<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    previous: SelectPreviousExperience<'a, 'a, Intv>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceMustBelongToOneOfPrevious<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        self.previous.add(experienced_event);
        Ok(())
    }

    fn result(&self) -> Result<()> {
        let Some(previous) = self.previous.as_ref() else {
            return Ok(());
        };

        let previous_afters = HashSet::<Id<Entity>>::from_iter(
            previous
                .experience
                .after
                .iter()
                .map(|profile| profile.entity),
        );

        if previous_afters.is_empty() {
            return Ok(());
        }

        if self
            .builder
            .before
            .as_ref()
            .map(|before| previous_afters.contains(&before.entity))
            .unwrap_or_default()
        {
            return Ok(());
        }

        Err(Error::NotInPreviousExperience)
    }
}

impl<'a, Intv> ExperienceMustBelongToOneOfPrevious<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            previous: SelectPreviousExperience::from_builder(builder),
        }
    }
}
