mod experience_kind_precedes_next;
pub use experience_kind_precedes_next::*;

mod experience_kind_follows_previous;
pub use experience_kind_follows_previous::*;

mod experience_belongs_to_one_of_previous;
pub use experience_belongs_to_one_of_previous::*;

mod experience_is_not_simultaneous;
pub use experience_is_not_simultaneous::*;

use crate::{
    experience::{ExperienceBuilder, ExperiencedEvent, Result},
    interval::Interval,
};

/// A Constraint is a condition that must be satified.
pub trait Constraint<'a, Intv> {
    /// Determines the constraint must take into account the given
    /// [ExperiencedEvent].
    ///
    /// Short-Circuiting: this method may return an error if, and only if, the
    /// given [ExperiencedEvent] already violates the constraint.
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()>;

    /// Returns the same error as `with`, if any. Otherwise returns the final
    /// veredict of the constraint.
    fn result(&self) -> Result<()>;
}

#[derive(Default)]
pub struct ConstraintGroup<'a, Intv> {
    constraints: Vec<Box<dyn Constraint<'a, Intv> + 'a>>,
}

impl<'a, Intv> ConstraintGroup<'a, Intv> {
    /// Inserts the given constraint in the constraint group.
    pub fn with_constraint(mut self, constraint: impl Constraint<'a, Intv> + 'a) -> Self {
        self.constraints.push(Box::new(constraint));
        self
    }

    /// Calls the [Constraint]'s result method consuming self.
    pub fn result(self) -> Result<()> {
        Constraint::<'a, Intv>::result(&self)
    }
}

impl<'a, Intv> Constraint<'a, Intv> for ConstraintGroup<'a, Intv> {
    fn with(&mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<()> {
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
            constraints: vec![
                Box::new(ExperienceKindFollowsPrevious::new(builder)),
                Box::new(ExperienceKindPrecedesNext::new(builder)),
                Box::new(ExperienceIsNotSimultaneous::new(builder)),
                Box::new(ExperienceBelongsToOneOfPrevious::new(builder)),
            ],
        }
    }
}
