use plotline::{
    experience::Experience,
    interval::Interval,
    plugin::{self, OutputError},
};

/// An entity cannot experience an event more than once.
pub const ALREADY_EXPERIENCED_ERROR: &str = "event_already_experienced";

pub struct EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    subject: &'a Experience<Intv>,
    already_experienced: bool,
}

impl<'a, Intv> EventIsNotExperiencedMoreThanOnce<'a, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experience: &'a Experience<Intv>) -> Self {
        self.already_experienced = self.subject.event == experience.event;
        self
    }

    pub fn result(&self) -> std::result::Result<(), OutputError> {
        if self.already_experienced {
            return Err(plugin::OutputError::new(ALREADY_EXPERIENCED_ERROR)
                .with_message("the entity would be experiencing the same event more than once"));
        }

        Ok(())
    }
}

impl<'a, Intv> EventIsNotExperiencedMoreThanOnce<'a, Intv> {
    pub fn new(subject: &'a Experience<Intv>) -> Self {
        Self {
            subject,
            already_experienced: false,
        }
    }
}
