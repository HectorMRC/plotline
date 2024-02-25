use super::{Constraint, Error, Recoverable, Result};
use crate::{error::ResidueError, event::Event, experience::ExperiencedEvent, interval::Interval};

pub struct EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    event: &'a Event<Intv>,
    already_experienced: bool,
}

impl<'a, Intv> Constraint<'a, Intv> for EventIsNotExperiencedMoreThanOnce<'a, Intv>
where
    Intv: Interval,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Recoverable<Self> {
        self.already_experienced = self.event == experienced_event.event;
        if self.already_experienced {
            return Err(ResidueError::new(self, Error::EventAlreadyExperienced));
        }

        Ok(self)
    }

    fn result(self) -> Result<()> {
        if self.already_experienced {
            return Err(Error::EventAlreadyExperienced);
        }

        Ok(())
    }
}

impl<'a, Intv> EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    pub fn new(event: &'a Event<Intv>) -> Self {
        Self {
            event,
            already_experienced: false,
        }
    }
}
