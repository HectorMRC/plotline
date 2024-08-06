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
