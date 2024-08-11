//! Definitions for creating and managing arbitrary rules.

pub mod chain;

/// Represents whatever rule that must be satisfied by a specific source.
pub trait Rule {
    /// The type over which the rule applies.
    type Source;
    /// The error type that may be returned by the rule.
    type Error;

    /// Returns true if, and only if, the given source satisfies the rule.
    fn matches(&self, source: &Self::Source) -> bool;

    /// Returns [Result::Ok] with the given source if, and only if, it does satifies the rule.
    /// Otherwise returns an error.
    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error>;
}

/// A [Rule] that only applies under specific conditions of the source.
pub trait TargetedRule: Rule {
    /// Returns true if, and only if, the rule applies on the give source.
    fn applies_on(&self, source: &Self::Source) -> bool;
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::Rule;

    /// A mock implementation of the [Rule] trait.
    #[derive(Debug, Default)]
    pub struct RuleMock<Source, Error> {
        pub matches_fn: Option<fn(source: &Source) -> bool>,
        pub must_match_fn: Option<fn(source: Source) -> Result<Source, Error>>,
    }

    impl<Source, Error> Rule for RuleMock<Source, Error> {
        type Source = Source;
        type Error = Error;

        fn matches(&self, source: &Self::Source) -> bool {
            self.matches_fn.expect("matches method must be set")(source)
        }

        fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error> {
            self.must_match_fn.expect("must_match method must be set")(source)
        }
    }

    impl<Source, Error> RuleMock<Source, Error> {
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
