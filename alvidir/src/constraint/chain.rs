use std::marker::PhantomData;

use super::Constraint;

/// A succession of arbitrary [Constraint]s that must be satisfied as a single one.
pub struct LiFoConstraintChain<Cnst, Head> {
    constraint: Cnst,
    head: Head,
}

impl<Cnst, Head> Constraint for LiFoConstraintChain<Cnst, Head>
where
    Head: Constraint<Source = Cnst::Source, Error = Cnst::Error>,
    Cnst: Constraint,
{
    type Source = Cnst::Source;
    type Error = Cnst::Error;

    fn matches(&self, source: &Self::Source) -> bool {
        self.constraint.matches(source) && self.head.matches(source)
    }

    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
        self.constraint
            .must_match(source)
            .and_then(|source| self.head.must_match(source))
    }
}

impl<Cnst, Head> LiFoConstraintChain<Cnst, Head>
where
    Head: Constraint<Source = Cnst::Source, Error = Cnst::Error>,
    Cnst: Constraint,
{
    /// Chains the given constraint with self.
    pub fn chain<Tail>(self, schema: Tail) -> LiFoConstraintChain<Tail, Self>
    where
        Tail: Constraint<Source = Cnst::Source, Error = Cnst::Error>,
    {
        LiFoConstraintChain {
            constraint: schema,
            head: self,
        }
    }
}

impl<Cnst> LiFoConstraintChain<Cnst, InfallibleConstraint<Cnst::Source, Cnst::Error>>
where
    Cnst: Constraint,
{
    /// Creates a new chain containing the given constraint.
    pub fn new(schema: Cnst) -> Self {
        Self {
            head: Default::default(),
            constraint: schema,
        }
    }
}

/// A [Constraint] implementation that never fails.
pub struct InfallibleConstraint<Src, Err> {
    source: PhantomData<Src>,
    error: PhantomData<Err>,
}

impl<Src, Err> Default for InfallibleConstraint<Src, Err> {
    fn default() -> Self {
        Self {
            source: PhantomData,
            error: PhantomData,
        }
    }
}

impl<Src, Err> Constraint for InfallibleConstraint<Src, Err> {
    type Source = Src;
    type Error = Err;

    fn matches(&self, _: &Self::Source) -> bool {
        true
    }

    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
        Ok(source)
    }
}
