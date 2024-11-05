/// Resource definition.

/// Represents a value in a source.
pub trait Resource {
    /// The type of source the resource is comming from.
    type Source;

    /// Retrives all the ocurrences of self in the source.
    fn all(source: &Self::Source) -> Vec<Self>
    where
        Self: Sized;
}
