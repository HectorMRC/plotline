use std::collections::HashSet;

use super::Constraint;
use crate::{
    experience::{ExperienceBuilder, ExperiencedEvent, Result, Experience},
    interval::Interval, id::Id, entity::Entity,
};
/// ExperienceMustBelongToOneOf makes sure no experience belongs to an [Entity]
/// that is not listed as one of the afters of the previous experience.
pub struct ExperienceMustBelongToOneOf<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    previous: Option<&'a Experience<Intv>>
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceMustBelongToOneOf<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        if experienced_event.event < self.builder.event {
            self.previous = Some(experienced_event.experience);
            return Ok(())
        }

        self.result()

        // if let Some(other) = befores.next() {
        //     let mut entities = vec![other.entity];
        //     if let Some(before) = self.before {
        //         entities.push(before.entity);
        //     }

        //     entities.extend(befores.map(|profile| profile.entity));
        //     return Err(Error::BeforeMustBeOneOf(entities));
        // }
    }

    fn result(&self) -> Result<()> {
        let Some(previous) = self.previous else {
            return Ok(());
        };

        let previous_afters: HashSet<Id<Entity>> = HashSet::from_iter(
            previous
                .after
                .iter()
                .map(|profile| {
                    profile.entity
                })
            );

        Ok(())
    }
}

impl<'a, Intv> ExperienceMustBelongToOneOf<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self { builder, previous: None, }
    }
}