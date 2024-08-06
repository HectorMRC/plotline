//! A constraint implementation for statically chaining arbitrary constraints.

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
    pub fn chain<Tail>(self, constraint: Tail) -> LiFoConstraintChain<Tail, Self>
    where
        Tail: Constraint<Source = Cnst::Source, Error = Cnst::Error>,
    {
        LiFoConstraintChain {
            constraint,
            head: self,
        }
    }
}

impl<Cnst> LiFoConstraintChain<Cnst, InfallibleConstraint<Cnst::Source, Cnst::Error>>
where
    Cnst: Constraint,
{
    /// Creates a new constrain chain with the given one, having [InfallibleConstraint] as the head
    /// of self.
    pub fn new(constraint: Cnst) -> Self {
        Self {
            head: Default::default(),
            constraint,
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

#[cfg(test)]
mod tests {
    use crate::constraint::Constraint;

    use super::LiFoConstraintChain;

    struct MustContains(char);

    impl Constraint for MustContains {
        type Source = &'static str;
        type Error = &'static str;

        fn matches(&self, source: &Self::Source) -> bool {
            source.contains(self.0)
        }

        fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
            if source.contains(self.0) {
                return Ok(source);
            }

            Err("the string does not contains the expected char")
        }
    }

    #[test]
    fn lifo_constraint_chain_must_run_all_constraints() {
        let constraint = LiFoConstraintChain::new(MustContains('a'))
            .chain(MustContains('1'))
            .chain(MustContains('ش'));

        struct Test {
            name: &'static str,
            subject: &'static str,
            matches: bool,
        }

        vec![
            Test {
                name: "subject failing all constraints should fail",
                subject: "hello world",
                matches: false,
            },
            Test {
                name: "subject failing one single constraint should fail",
                subject: "a1",
                matches: false,
            },
            Test {
                name: "subject fulfilling all constraints should success",
                subject: "a1ش",
                matches: true,
            },
        ]
        .into_iter()
        .for_each(|test| {
            let matches = constraint.matches(&test.subject);
            assert_eq!(
                matches, test.matches,
                "{} got matches = {matches}, want {}",
                test.name, test.matches
            )
        })
    }
}
