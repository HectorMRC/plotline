use crate::{
    event::Event,
    experience::{ExperienceBuilder, ExperiencedEvent},
    interval::Interval,
};
use std::{cmp, ops::Deref};

pub struct SelectNextExperience<'a, 'b, Intv> {
    event: &'a Event<Intv>,
    next: Option<&'b ExperiencedEvent<'b, Intv>>,
}

impl<'a, 'b, Intv> Deref for SelectNextExperience<'a, 'b, Intv> {
    type Target = Option<&'b ExperiencedEvent<'b, Intv>>;

    fn deref(&self) -> &Self::Target {
        &self.next
    }
}

impl<'a, 'b, Intv> SelectNextExperience<'a, 'b, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experienced_event: &'b ExperiencedEvent<'b, Intv>) -> Self {
        self.add(experienced_event);
        self
    }

    pub fn add(&mut self, experienced_event: &'b ExperiencedEvent<'b, Intv>) {
        if experienced_event.event.hi() > self.event.lo() {
            self.next = cmp::min(self.next, Some(experienced_event)).or(Some(experienced_event));
        }
    }
}

impl<'a, 'b, Intv> SelectNextExperience<'a, 'b, Intv> {
    pub fn from_builder(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        SelectNextExperience {
            event: builder.event,
            next: None,
        }
    }

    pub fn value(self) -> Option<&'b ExperiencedEvent<'b, Intv>> {
        self.next
    }
}
