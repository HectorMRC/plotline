use crate::{event::Event, experience::Experience, interval::Interval};
use std::{cmp, ops::Deref};

pub struct SelectNextExperience<'a, 'b, Intv> {
    event: &'a Event<Intv>,
    next: Option<&'b Experience<Intv>>,
}

impl<'a, 'b, Intv> Deref for SelectNextExperience<'a, 'b, Intv> {
    type Target = Option<&'b Experience<Intv>>;

    fn deref(&self) -> &Self::Target {
        &self.next
    }
}

impl<'a, 'b, Intv> SelectNextExperience<'a, 'b, Intv>
where
    Intv: Interval,
{
    pub fn with(mut self, experience: &'b Experience<Intv>) -> Self {
        self.add(experience);
        self
    }

    pub fn add(&mut self, experience: &'b Experience<Intv>) {
        if experience.event.hi() > self.event.lo() {
            self.next = cmp::min(self.next, Some(experience)).or(Some(experience));
        }
    }
}

impl<'a, 'b, Intv> SelectNextExperience<'a, 'b, Intv> {
    pub fn new(event: &'a Event<Intv>) -> Self {
        SelectNextExperience { event, next: None }
    }

    pub fn value(self) -> Option<&'b Experience<Intv>> {
        self.next
    }
}
