use super::Constraint;
use crate::{
    experience::{Error, ExperienceBuilder, ExperienceKind, ExperiencedEvent, Result},
    interval::Interval,
};

pub struct InitialMustBeBeforeOrEqual<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
}

impl<'a, Intv> Constraint<Intv> for InitialMustBeBeforeOrEqual<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()> {
        let kind: ExperienceKind = experienced_event.experience.into();
        if kind.is_initial() && self.builder.event < experienced_event.event {
            return Err(Error::BeforeInitial);
        }

        Ok(())
    }

    fn result(&self) -> Result<()> {
        Ok(())
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self { builder }
    }
}
