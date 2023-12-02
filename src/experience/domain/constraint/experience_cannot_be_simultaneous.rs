use super::Constraint;
use crate::{
    experience::{Error, ExperienceBuilder, ExperiencedEvent, Result},
    interval::Interval,
};

pub struct ExperienceCannotBeSimultaneous<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    conflict: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> Constraint<'a, Intv> for ExperienceCannotBeSimultaneous<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
        if self.builder.event.intersects(experienced_event.event) {
            self.conflict = Some(experienced_event);
        }

        self.result()
    }

    fn result(&self) -> Result<()> {
        if self.conflict.is_some() {
            return Err(Error::SimultaneousEvents);
        }

        Ok(())
    }
}
