use crate::{
    event::Event,
    experience::{ExperienceBuilder, ExperiencedEvent},
    interval::Interval,
};
use std::cmp;

/// SelectCloserExperiences implements the filter that selects from an iterator
/// of [ExperiencedEvent]s those two happening immediately before and after the
/// given [Event].
pub struct SelectCloserExperiences<'a, 'b: 'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<&'b ExperiencedEvent<'b, Intv>>,
    after: Option<&'b ExperiencedEvent<'b, Intv>>,
}

impl<'a, 'b: 'a, Intv> SelectCloserExperiences<'a, 'b, Intv> {
    pub fn from_builder(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        SelectCloserExperiences {
            event: builder.event,
            before: None,
            after: None,
        }
    }
}

impl<'a, 'b, Intv> SelectCloserExperiences<'a, 'b, Intv>
where
    'b: 'a,
    Intv: Interval,
{
    /// Consumes the iterator selecting from it those [ExperiencedEvent]s
    /// happening immediately before and after of the given [Event].
    ///
    /// Calling to `with_iter` multiple times will end up with the closest
    /// before and after among all the consumed iterators.
    pub fn with_iter(mut self, iter: impl Iterator<Item = &'b ExperiencedEvent<'b, Intv>>) -> Self {
        iter.for_each(|experienced_event| {
            if experienced_event.event.hi() < self.event.lo() {
                self.before = cmp::max(self.before, Some(experienced_event));
            } else if experienced_event.event.lo() > self.event.hi() {
                self.after = cmp::min(self.after, Some(experienced_event));
            }
        });

        self
    }
}

impl<'a, 'b: 'a, Intv> SelectCloserExperiences<'a, 'b, Intv> {
    /// Returns the tuple containing the selected before and after.
    pub fn result(
        self,
    ) -> (
        Option<&'b ExperiencedEvent<'b, Intv>>,
        Option<&'b ExperiencedEvent<'b, Intv>>,
    ) {
        (self.before, self.after)
    }
}
