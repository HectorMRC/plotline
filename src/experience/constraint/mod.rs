mod experience_kind_precedes_next;
pub use experience_kind_precedes_next::*;

mod experience_kind_follows_previous;
pub use experience_kind_follows_previous::*;

mod experience_belongs_to_one_of_previous;
pub use experience_belongs_to_one_of_previous::*;

mod experience_is_not_simultaneous;
pub use experience_is_not_simultaneous::*;

use crate::{experience::ExperiencedEvent, interval::Interval};
use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error, Clone)]
pub enum Error {
    #[error("an experience cannot belong to an entity not listed in the previous experience")]
    NotInPreviousExperience,
    #[error("an entity cannot experience simultaneous events")]
    SimultaneousEvents,
    #[error("a terminal experience cannot follows a terminal one")]
    TerminalFollowsTerminal,
    #[error("a terminal experience cannot precede a terminal one")]
    TerminalPrecedesTerminal,
    #[error("{0:?}")]
    Stack(Vec<Error>),
    #[error("{0}")]
    Custom(&'static str),
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Error {
        value.cause
    }
}

impl Error {
    pub fn push(self, tail: Self) -> Self {
        Error::Stack(match (self, tail) {
            (Error::Stack(mut head_errors), Error::Stack(tail_errors)) => {
                head_errors.extend(tail_errors);
                head_errors
            }
            (Error::Stack(mut head_errors), tail_error) => {
                head_errors.push(tail_error);
                head_errors
            }
            (head_error, Error::Stack(tail_errors)) => {
                let mut tmp = vec![head_error];
                tmp.extend(tail_errors);
                tmp
            }
            (head_error, tail_error) => vec![head_error, tail_error],
        })
    }
}

pub type ConstraintResult<Cnst> = std::result::Result<Cnst, PoisonError<Cnst>>;

#[derive(Debug)]
pub struct PoisonError<T> {
    pub cause: Error,
    pub guard: T,
}

impl<T: Debug> std::error::Error for PoisonError<T> {}

impl<T> Display for PoisonError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.cause)
    }
}

impl<T> PoisonError<T> {
    pub fn new(guard: T, cause: Error) -> Self {
        PoisonError { guard, cause }
    }

    pub fn into_inner(self) -> T {
        self.guard
    }
}

/// A Constraint is a condition that must be satified.
pub trait Constraint<'a, Intv>: Sized {
    /// Determines the constraint must take into account the given
    /// [ExperiencedEvent].
    ///
    /// Short-Circuiting: this method may return an error if, and only if, the
    /// given [ExperiencedEvent] already violates the constraint.
    fn with(self, experienced_event: &'a ExperiencedEvent<Intv>) -> ConstraintResult<Self>;

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
    early: bool,
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
            early: self.early,
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
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> ConstraintResult<Self> {
        let evaluate_head = |mut chain: Self, tail_error| match chain
            .head
            .map(|cnst| cnst.with(experienced_event))
            .transpose()
        {
            Ok(head) => {
                chain.head = head;
                if let Some(error) = tail_error {
                    return Err(PoisonError::new(chain, error));
                }

                Ok(chain)
            }
            Err(poison_err) => {
                chain.head = Some(poison_err.guard);
                let mut error = poison_err.cause;

                if let Some(tail_error) = tail_error {
                    error = error.push(tail_error);
                }

                Err(PoisonError::new(chain, error))
            }
        };

        match self.constraint.with(experienced_event) {
            Ok(constraint) => {
                self.constraint = constraint;
                evaluate_head(self, None)
            }

            Err(poison_err) => {
                self.constraint = poison_err.guard;
                let error = poison_err.cause;

                if self.early {
                    return Err(PoisonError::new(self, error));
                }

                evaluate_head(self, Some(error))
            }
        }
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
            early: true,
        }
    }

    pub fn with_early(mut self, enable: bool) -> Self {
        self.early = enable;
        self
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

/// Implement [Constraint] for () so [LiFoConstraintChain] can use it as the
/// default type of Head and Cnst.
impl<'a, Intv> Constraint<'a, Intv> for () {
    fn with(self, _: &'a ExperiencedEvent<Intv>) -> ConstraintResult<Self> {
        Ok(self)
    }

    fn result(self) -> Result<()> {
        Ok(())
    }
}

/// InhibitableConstraint decorates a [Constraint] to inhibit some of its errors.
pub struct InhibitableConstraint<Cnst, Inh> {
    constraint: Cnst,
    inhibitors: Vec<Inh>,
}

impl<'a, Intv, Cnst, Inh> Constraint<'a, Intv> for InhibitableConstraint<Cnst, Inh>
where
    Cnst: Constraint<'a, Intv>,
    Inh: PartialEq<Error>,
{
    fn with(mut self, experienced_event: &'a ExperiencedEvent<Intv>) -> ConstraintResult<Self> {
        match self.constraint.with(experienced_event) {
            Ok(constraint) => {
                self.constraint = constraint;
                Ok(self)
            }
            Err(poison_err) => {
                self.constraint = poison_err.guard;
                if self
                    .inhibitors
                    .iter()
                    .any(|inhibitor| inhibitor == &poison_err.cause)
                {
                    return Ok(self);
                }

                Err(PoisonError::new(self, poison_err.cause))
            }
        }
    }

    fn result(self) -> Result<()> {
        match self.constraint.result() {
            Err(err) if self.inhibitors.iter().any(|inhibitor| inhibitor == &err) => Ok(()),
            other => other,
        }
    }
}

impl<Cnst, Inh> InhibitableConstraint<Cnst, Inh> {
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
