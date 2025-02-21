//! Property definition.

/// A value in a source.
pub trait Property<Src>: Sized {
    /// Retrives all the ocurrences of self in the source.
    fn all(source: &Src) -> Vec<Self>;
}
