use crate::{
    experience::{Experience, ExperienceBuilder, ExperienceKind, ExperiencedEvent, Result, Error},
    interval::Interval,
};
use std::cmp;

/// Creates a new experience caused by the given event as long as it fits in
/// the given ordered succession of experienced events.
pub fn create<'a, Intv: Interval>(
    builder: ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) -> Result<Experience<Intv>> {
    

    {
        let mut closer_experiences = SelectCloserExperiences::new(&builder);
        let mut constaints_group = ConstraintGroup::with_defaults(&builder);

        experienced_events
            .iter()
            .try_for_each(|experienced_event| {
                closer_experiences.with(experienced_event);
                constaints_group.with(experienced_event)
            })?;

        constaints_group.result()?;
    }

    builder.build()
}

/// Tries to fill the optional fields of the given builder that are set to [Option::None].
pub fn fill_optional_fields<'a, Intv: Interval>(
    builder: &mut ExperienceBuilder<'a, Intv>,
    experienced_events: &[ExperiencedEvent<'a, Intv>],
) {
    let closer_experiences = SelectCloserExperiences::new(&builder).with_iter(experienced_events.iter());
    builder.before = builder.before.or(closer_experiences.before.map(|experienced_event| experienced_event.experience.after));
}

struct SelectCloserExperiences<'a, Intv> {
    builder: &'a ExperienceBuilder<'a, Intv>,
    before: Option<&'a ExperiencedEvent<'a, Intv>>,
    after: Option<&'a ExperiencedEvent<'a, Intv>>,
}

impl<'a, Intv> SelectCloserExperiences<'a, Intv> {
    fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        SelectCloserExperiences {
            builder,
            before: None,
            after: None,
        }
    }
}

impl<'a, Intv> SelectCloserExperiences<'a, Intv>
where
    Intv: Interval,
{
    /// Consumes the iterator selecting from it the closest experiences to the subject event.
    fn with_iter(mut self, iter: impl Iterator<Item = &'a ExperiencedEvent<'a, Intv>>) -> Self {
        iter.for_each(|experienced_event| {
            if experienced_event.event.hi() < self.builder.event.lo() {
                self.before = cmp::max(self.before, Some(experienced_event));
            } else if experienced_event.event.lo() > self.builder.event.hi() {
                self.after = cmp::min(self.after, Some(experienced_event));
            }
        });

        self  
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
    Intv: Interval
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
    found: bool,
}

impl<'a, Intv> Constraint<Intv> for InitialMustBeBeforeOrEqual<'a, Intv>
where
    Intv: Interval
{
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()> {
        let kind: ExperienceKind = experienced_event.experience.into();
        self.found |= kind.is_initial();

        if kind.is_initial() && self.builder.event < experienced_event.event {
            return Err(Error::BeforeInitial);
        }

        Ok(())
    }

    fn result(&self) -> Result<()> {
        if self.found {
            Ok(())
        } else {
            Err(Error::NoInitial)
        }
    }
}

impl<'a, Intv> InitialMustBeBeforeOrEqual<'a, Intv> {
    fn new(builder: &'a ExperienceBuilder<'a, Intv>) -> Self {
        Self {
            builder,
            found: false,
        }
    }
}
