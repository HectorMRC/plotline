use crate::{
    event::Event,
    experience::{Experience, ExperienceBuilder, ExperiencedEvent, Result},
    id::Identifiable,
    interval::Interval,
};
use std::cmp;

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<Intv: Interval>(
    event: &Event<Intv>,
    experienced_events: &[ExperiencedEvent<'_, Intv>],
) -> Result<Experience<Intv>> {
    let mut closer_experiences = SelectCloserExperiences::new(event);
    let mut constaints_group = ConstraintGroup::new(event);

    experienced_events.iter().try_for_each(|experienced_event| {
        closer_experiences.with(experienced_event);
        constaints_group.with(experienced_event)
    })?;

    constaints_group.result()?;
    ExperienceBuilder::new(event.id()).build()
}

struct SelectCloserExperiences<'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<&'a ExperiencedEvent<'a, Intv>>,
    after: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> SelectCloserExperiences<'a, Intv>
where
    Intv: Interval,
{
    fn new(event: &'a Event<Intv>) -> Self {
        SelectCloserExperiences {
            event,
            before: None,
            after: None,
        }
    }

    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) {
        if experienced_event.event.hi() < self.event.lo() {
            self.before = cmp::max(self.before, Some(experienced_event));
        } else if experienced_event.event.lo() > self.event.hi() {
            self.after = cmp::min(self.after, Some(experienced_event));
        }
    }
}

trait Constraint<Intv> {
    fn with(&mut self, experienced_event: &'_ ExperiencedEvent<Intv>) -> Result<()>;
    fn result(self) -> Result<()>;
}

struct ConstraintGroup<'a, Intv> {
    constraints: Vec<Box<dyn Constraint<Intv> + 'a>>,
}

impl<'a, Intv> Constraint<Intv> for ConstraintGroup<'a, Intv> {
    fn with(&mut self, experienced_event: &'_ ExperiencedEvent<Intv>) -> Result<()> {
        todo!()
    }

    fn result(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, Intv> ConstraintGroup<'a, Intv> {
    fn new(event: &'a Event<Intv>) -> Self {
        Self {
            constraints: vec![Box::new(InitialMustBeBeforeOrEqual::new(event))],
        }
    }
}

struct InitialMustBeBeforeOrEqual<'a, Intv> {
    event: &'a Event<Intv>,
}

impl<'a, Intv> Constraint<Intv> for InitialMustBeBeforeOrEqual<'a, Intv> {
    fn with(&mut self, experienced_event: &'_ ExperiencedEvent<Intv>) -> Result<()> {
        todo!()
    }

    fn result(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    fn new(event: &'a Event<Intv>) -> Self {
        Self { event }
    }
}
