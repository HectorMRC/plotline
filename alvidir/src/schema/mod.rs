pub mod chain;

/// Defines the structure and validation rules for an specific source.
pub trait Schema {
    /// The type over which the schema applies.
    type Source;
    /// The error type that may be returned by the schema.
    type Error;

    /// Returns true if, and only if, the given source satisfies the schema.
    fn matches(&self, source: &Self::Source) -> bool;

    /// Returns [Result::Ok] with the given source if, and only if, it does satifies the schema.
    /// Otherwise returns an error.
    fn must_match(&self, source: Self::Source) -> Result<Self::Source, Self::Error>;
}
