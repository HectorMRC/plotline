mod initial_must_be_before_or_equal;
pub use initial_must_be_before_or_equal::*;

use crate::{
    experience::{ExperienceBuilder, ExperiencedEvent, Result},
    interval::Interval,
};

pub trait Constraint<Intv> {
    fn with(&mut self, experienced_event: &ExperiencedEvent<Intv>) -> Result<()>;
    fn result(&self) -> Result<()>;
}

#[derive(Default)]
pub struct ConstraintGroup<'a, Intv> {
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
