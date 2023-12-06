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
pub trait Constraint<'a, Intv>: Sized {
    /// Determines the constraint must take into account the given
    /// [ExperiencedEvent].
    ///
    /// Short-Circuiting: this method may return an error if, and only if, the
    /// given [ExperiencedEvent] already violates the constraint.
    fn with(self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self>;

    /// Returns the same error as `with`, if any. Otherwise returns the final
    /// veredict of the constraint.
    fn result(self) -> Result<()>;
}

/// A ConstraintChain is a succession of [Constraint]s that must be satified as
/// a whole.
pub trait ConstraintChain<'a, Intv>: Constraint<'a, Intv> {
    type Link<Cnst>: ConstraintChain<'a, Intv>
    where
        Cnst: Constraint<'a, Intv>;

    /// Chains the given [Constraint] with self.
    fn chain<Cnst>(self, constraint: Cnst) -> Self::Link<Cnst>
    where
        Cnst: Constraint<'a, Intv>;
}

/// ConstraintLink implements the [ConstraintChain], allowing to chain
/// different implementations of [Constraint].
pub struct ConstraintLink<Cnst1, Cnst2> {
    previous: Option<Cnst1>,
    constraint: Cnst2,
}

impl<'a, Intv, Cnst1, Cnst2> ConstraintChain<'a, Intv> for ConstraintLink<Cnst1, Cnst2>
where
    Cnst1: Constraint<'a, Intv>,
    Cnst2: Constraint<'a, Intv>,
{
    type Link<Cnst3> = ConstraintLink<Self, Cnst3>
        where Cnst3: Constraint<'a, Intv>;

    fn chain<Cnst3>(self, constraint: Cnst3) -> Self::Link<Cnst3>
    where
        Cnst3: Constraint<'a, Intv>,
    {
        ConstraintLink {
            previous: Some(self),
            constraint,
        }
    }
}

impl<'a, Intv, Cnst1, Cnst2> Constraint<'a, Intv> for ConstraintLink<Cnst1, Cnst2>
where
    Cnst1: Constraint<'a, Intv>,
    Cnst2: Constraint<'a, Intv>,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        self.previous = self
            .previous
            .map(|cnst| cnst.with(experienced_event))
            .transpose()?;

        self.constraint = self.constraint.with(experienced_event)?;
        Ok(self)
    }

    fn result(self) -> Result<()> {
        self.previous.map(|cnst| cnst.result()).transpose()?;
        self.constraint.result()
    }
}

impl<Cnst> ConstraintLink<(), Cnst> {
    pub fn new(constraint: Cnst) -> Self {
        Self {
            previous: None,
            constraint,
        }
    }
}

impl ConstraintLink<(), ()> {
    /// Creates a [ConstraintChain] with the default [Constraint]s.
    pub fn with_defaults<'a, Intv>(
        builder: &'a ExperienceBuilder<'a, Intv>,
    ) -> impl ConstraintChain<'a, Intv>
    where
        Intv: Interval,
    {
        ConstraintLink::new(ExperienceIsNotSimultaneous::new(builder))
            .chain(ExperienceBelongsToOneOfPrevious::new(builder))
            .chain(ExperienceKindFollowsPrevious::new(builder))
            .chain(ExperienceKindPrecedesNext::new(builder))
    }
}

impl<'a, Intv> Constraint<'a, Intv> for () {
    fn with(self, _: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        Ok(self)
    }

    fn result(self) -> Result<()> {
        Ok(())
    }
}