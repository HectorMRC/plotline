use super::Constraint;
use crate::{
    entity::Entity,
    experience::{Error, ExperienceBuilder, ExperiencedEvent, Result},
    id::Id,
    interval::Interval,
};
use std::{cmp, collections::HashSet};

/// ExperienceMustBelongToOneOf makes sure no experience belongs to an [Entity]
/// that is not listed as one of the afters of the previous experience.
pub struct ExperienceMustBelongToOneOf<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    previous: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceMustBelongToOneOf<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) {
        if experienced_event.event < self.builder.event {
            self.previous = cmp::max(self.previous, Some(experienced_event))
        }
    }

    fn result(&self) -> Result<()> {
        let Some(previous) = self.previous else {
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

        Err(Error::ExperienceMustBelongToOneOf(
            previous_afters.into_iter().collect(),
        ))
    }
}

impl<'a, Intv> ExperienceMustBelongToOneOf<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            previous: None,
        }
    }
}
