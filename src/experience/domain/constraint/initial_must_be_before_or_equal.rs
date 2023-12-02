use super::Constraint;
use crate::{
    experience::{Error, ExperienceBuilder, ExperienceKind, ExperiencedEvent, Result},
    interval::Interval,
};

/// InitialMustBeBeforeOrEqual makes sure no experience takes places before the
/// initial experience of the corresponding [Entity].
pub struct InitialMustBeBeforeOrEqual<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    initial: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for InitialMustBeBeforeOrEqual<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        let kind: ExperienceKind = experienced_event.experience.into();
        if kind.is_initial() {
            self.initial = Some(experienced_event);
        }

        self.result()
    }

    fn result(&self) -> Result<()> {
        let Some(initial) = self.initial else {
            return Ok(());
        };

        if self.builder.event < initial.event {
            return Err(Error::ExperienceBeforeInitial);
        }

        Ok(())
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    pub fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            initial: None,
        }
    }
}
