//! Definitions for creating and managing arbitrary constraints.

pub mod chain;

/// Represents whatever rule that must be satisfied by a specific source.
pub trait Constraint {
    /// The type over which the constraint applies.
    type Source;
    /// The error type that may be returned by the constraint.
    type Error;

    /// Returns true if, and only if, the given source satisfies the constraint.
    fn matches(&self, source: &Self::Source) -> bool;

    /// Returns [Result::Ok] with the given source if, and only if, it does satifies the constraint.
    /// Otherwise returns an error.
    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error>;
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::Constraint;

    /// A mock implementation of the [Constraint] trait.
    #[derive(Debug, Default)]
    pub struct ConstraintMock<Source, Error> {
        pub matches_fn: Option<fn(source: &Source) -> bool>,
        pub must_match_fn: Option<fn(source: Source) -> Result<Source, Error>>,
    }

    impl<Source, Error> Constraint for ConstraintMock<Source, Error> {
        type Source = Source;
        type Error = Error;

        fn matches(&self, source: &Self::Source) -> bool {
            self.matches_fn.expect("matches method must be set")(source)
        }

        fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
            self.must_match_fn.expect("must_match method must be set")(source)
        }
    }

    impl<Source, Error> ConstraintMock<Source, Error> {
        pub fn with_matches_fn(mut self, f: fn(&Source) -> bool) -> Self {
            self.matches_fn = Some(f);
            self
        }

        pub fn with_must_match_fn(mut self, f: fn(Source) -> Result<Source, Error>) -> Self {
            self.must_match_fn = Some(f);
            self
        }
    }
}
