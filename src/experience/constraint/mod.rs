mod experience_kind_precedes_next;
pub use experience_kind_precedes_next::*;

mod experience_kind_follows_previous;
pub use experience_kind_follows_previous::*;

mod experience_belongs_to_one_of_previous;
pub use experience_belongs_to_one_of_previous::*;

mod experience_is_not_simultaneous;
pub use experience_is_not_simultaneous::*;

use crate::{experience::ExperiencedEvent, interval::Interval};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error, Clone, Copy)]
pub enum Error {
    #[error("an experience cannot belong to an entity not listed in the previous experience")]
    NotInPreviousExperience,
    #[error("an entity cannot experience simultaneous events")]
    SimultaneousEvents,
    #[error("a terminal experience cannot follows a terminal one")]
    TerminalFollowsTerminal,
    #[error("a terminal experience cannot precede a terminal one")]
    TerminalPrecedesTerminal,
}

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

/// LiFoConstraintChain implements a _last-in first-out_ [ConstraintChain] that
/// allows different implementations of [Constraint] to be chained into a
/// single one.
pub struct LiFoConstraintChain<Head, Cnst> {
    head: Option<Head>,
    constraint: Cnst,
}

impl<'a, Intv, Head, Cnst> ConstraintChain<'a, Intv> for LiFoConstraintChain<Head, Cnst>
where
    Head: Constraint<'a, Intv>,
    Cnst: Constraint<'a, Intv>,
{
    type Link<Tail> = LiFoConstraintChain<Self, Tail>
        where Tail: Constraint<'a, Intv>;

    fn chain<Tail>(self, constraint: Tail) -> Self::Link<Tail>
    where
        Tail: Constraint<'a, Intv>,
    {
        LiFoConstraintChain {
            head: Some(self),
            constraint,
        }
    }
}

impl<'a, Intv, Head, Cnst> Constraint<'a, Intv> for LiFoConstraintChain<Head, Cnst>
where
    Head: Constraint<'a, Intv>,
    Cnst: Constraint<'a, Intv>,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        self.constraint = self.constraint.with(experienced_event)?;
        self.head = self
            .head
            .map(|cnst| cnst.with(experienced_event))
            .transpose()?;

        Ok(self)
    }

    fn result(self) -> Result<()> {
        self.constraint.result()?;
        self.head.map(|cnst| cnst.result()).transpose()?;
        Ok(())
    }
}

impl<Cnst> LiFoConstraintChain<(), Cnst> {
    pub fn new(constraint: Cnst) -> Self {
        Self {
            head: None,
            constraint,
        }
    }
}

impl LiFoConstraintChain<(), ()> {
    /// Creates a [ConstraintChain] with the default [Constraint]s.
    pub fn with_defaults<'a, Intv>(
        experienced_event: &'a ExperiencedEvent<'a, Intv>,
    ) -> impl ConstraintChain<'a, Intv>
    where
        Intv: Interval,
    {
        LiFoConstraintChain::new(ExperienceBelongsToOneOfPrevious::new(experienced_event))
            .chain(ExperienceKindFollowsPrevious::new(experienced_event))
            .chain(ExperienceKindPrecedesNext::new(experienced_event))
            .chain(ExperienceIsNotSimultaneous::new(experienced_event.event))
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

/// InhibitedConstraint decorates a [Constraint] to inhibit some of its errors.
pub struct InhibitedConstraint<Cnst, Inh> {
    constraint: Cnst,
    inhibitors: Vec<Inh>,
}

impl<'a, Intv, Cnst, Inh> Constraint<'a, Intv> for InhibitedConstraint<Cnst, Inh>
where
    Cnst: Constraint<'a, Intv> + Clone,
    Inh: PartialEq<Error>,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> Result<Self> {
        let backup = self.constraint.clone();
        self.constraint = match self.constraint.with(experienced_event) {
            Err(err) if self.inhibitors.iter().any(|inhibitor| inhibitor == &err) => Ok(backup),
            other => other,
        }?;

        Ok(self)
    }

    fn result(self) -> Result<()> {
        match self.constraint.result() {
            Err(err) if self.inhibitors.iter().any(|inhibitor| inhibitor == &err) => Ok(()),
            other => other,
        }
    }
}

impl<Cnst, Inh> InhibitedConstraint<Cnst, Inh> {
    pub fn new(constraint: Cnst) -> Self {
        Self {
            constraint,
            inhibitors: Vec::default(),
        }
    }

    pub fn with_inhibitor(mut self, inhibitor: Inh) -> Self {
        self.inhibitors.push(inhibitor);
        self
    }
}
