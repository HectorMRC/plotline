use crate::{
    entity::Entity,
    event::Event,
    experience::{Error, Experience, ExperienceBuilder, ExperienceKind, ExperiencedEvent, Result},
    id::Id,
    interval::Interval,
};
use std::{cmp, collections::HashSet};

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<'a, Intv: Interval>(
    builder: ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) -> Result<Experience<Intv>> {
    let builder = builder.with_fallbacks(experienced_events)?;

    {
        let mut constaints_group = ConstraintGroup::with_defaults(&builder);
        experienced_events
            .iter()
            .try_for_each(|experienced_event| constaints_group.with(experienced_event))?;

        constaints_group.result()?;
    }

    builder.build()
}

impl<'a, Intv> ExperienceBuilder<'a, Intv>
where
    Intv: Interval,
{
    /// Tries to compute some value for any field set to [Option::None].
    fn with_fallbacks(mut self, experienced_events: &[ExperiencedEvent<'a, Intv>]) -> Result<Self> {
        if self.before.is_some() && self.after.is_some() {
            return Ok(self);
        }

        let (before, after) = SelectCloserExperiences::from_builder(&self)
            .with_iter(experienced_events.iter())
            .result();

        if self.after.is_none() {
            self.after = after
                .and_then(|experienced_event| experienced_event.experience.before.clone())
                .map(|before| vec![before]);
        }

        let afters: HashSet<Id<Entity>> = self
            .after
            .as_ref()
            .map(|experienced_events| {
                HashSet::from_iter(experienced_events.iter().map(|profile| profile.entity))
            })
            .unwrap_or_default();

        let mut befores = before
            .map(|experienced_event| experienced_event.experience.after.clone())
            .unwrap_or_default()
            .into_iter()
            .filter(|profile| afters.contains(&profile.entity));

        self.before = befores.next();
        if befores.next().is_some() {
            return Err(Error::BeforeIsRequired);
        }

        Ok(self)
    }
}

/// SelectCloserExperiences implements the filter that selects from an iterator
/// of [ExperiencedEvent]s those two happening immediately before and after the
/// given [Event].
struct SelectCloserExperiences<'a, 'b: 'a, Intv> {
    event: &'a Event<Intv>,
    before: Option<&'b ExperiencedEvent<'b, Intv>>,
    after: Option<&'b ExperiencedEvent<'b, Intv>>,
}

impl<'a, 'b: 'a, Intv> SelectCloserExperiences<'a, 'b, Intv> {
    fn from_builder(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
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
    fn with_iter(mut self, iter: impl Iterator<Item = &'b ExperiencedEvent<'b, Intv>>) -> Self {
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
    fn result(
        self,
    ) -> (
        Option<&'b ExperiencedEvent<'b, Intv>>,
        Option<&'b ExperiencedEvent<'b, Intv>>,
    ) {
        (self.before, self.after)
    }
}

trait Constraint<Intv> {
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()>;
    fn result(&self) -> Result<()>;
}

#[derive(Default)]
struct ConstraintGroup<'a, Intv> {
    constraints: Vec<Box<dyn Constraint<Intv> + 'a>>,
}

impl<'a, Intv> ConstraintGroup<'a, Intv> {
    /// Inserts the given constraint in the constraint group.
    pub fn _with_constraint(mut self, constraint: impl Constraint<Intv> + 'a) -> Self {
        self.constraints.push(Box::new(constraint));
        self
    }
}

impl<'a, Intv> Constraint<Intv> for ConstraintGroup<'a, Intv> {
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()> {
        self.constraints
            .iter_mut()
            .try_for_each(|constraint| constraint.with(experienced_event))
    }

    fn result(&self) -> Result<()> {
        self.constraints
            .iter()
            .try_for_each(|constraint| constraint.result())
    }
}

impl<'a, Intv> ConstraintGroup<'a, Intv>
where
    Intv: Interval,
{
    /// Creates a [ConstraintGroup] with all the default constraints.
    pub fn with_defaults(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            constraints: vec![Box::new(InitialMustBeBeforeOrEqual::new(builder))],
        }
    }
}

struct InitialMustBeBeforeOrEqual<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
}

impl<'a, Intv> Constraint<Intv> for InitialMustBeBeforeOrEqual<'a, Intv>
where
    Intv: Interval,
{
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()> {
        let kind: ExperienceKind = experienced_event.experience.into();
        if kind.is_initial() && self.builder.event < experienced_event.event {
            return Err(Error::BeforeInitial);
        }

        Ok(())
    }

    fn result(&self) -> Result<()> {
        Ok(())
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self { builder }
    }
}
