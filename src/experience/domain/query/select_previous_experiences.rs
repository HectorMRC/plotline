use crate::{
    event::Event,
    experience::{ExperienceBuilder, ExperiencedEvent},
    interval::Interval,
};
use std::{cmp, ops::Deref};

pub struct SelectPreviousExperience<'a, 'b, Intv> {
    event: &'a Event<Intv>,
    previous: Option<&'b ExperiencedEvent<'b, Intv>>,
}

impl<'a, 'b, Intv> Deref for SelectPreviousExperience<'a, 'b, Intv> {
    type Target = Option<&'b ExperiencedEvent<'b, Intv>>;

    fn deref(&self) -> &Self::Target {
        &self.previous
    }
}

impl<'a, 'b, Intv> SelectPreviousExperience<'a, 'b, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experienced_event: &'b ExperiencedEvent<'b, Intv>) -> Self {
        self.add(experienced_event);
        self
    }

    pub fn add(&mut self, experienced_event: &'b ExperiencedEvent<'b, Intv>) {
        if experienced_event.event.hi() < self.event.lo() {
            self.previous = cmp::max(self.previous, Some(experienced_event));
        }
    }
}

impl<'a, 'b, Intv> SelectPreviousExperience<'a, 'b, Intv> {
    pub fn from_builder(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        SelectPreviousExperience {
            event: builder.event,
            previous: None,
        }
    }

    pub fn value(self) -> Option<&'b ExperiencedEvent<'b, Intv>> {
        self.previous
    }
}
