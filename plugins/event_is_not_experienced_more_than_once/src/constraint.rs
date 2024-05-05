use plotline::{
    experience::Experience,
    id::Indentify,
    interval::Interval,
    plugin::{self, PluginError},
};

/// An entity cannot experience an event more than once.
pub const ALREADY_EXPERIENCED_ERROR: &str = "EventAlreadyExperienced";

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

    pub fn result(&self) -> std::result::Result<(), PluginError> {
        if self.already_experienced {
            return Err(
                plugin::PluginError::new(ALREADY_EXPERIENCED_ERROR).with_message(format!(
                    "the entity {} would be experiencing the event {} more than once",
                    self.subject.entity.id(),
                    self.subject.event.id(),
                )),
            );
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
