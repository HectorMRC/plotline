//! Property definition.

/// A value in a source.
pub trait Property<Src>: Sized {
    /// Retrives all the ocurrences of self in the source.
    fn all(source: &Src) -> Vec<Self>;
}

/// An entity that is able to extract ocurrences of a target type in a source.
pub trait Extract<Src> {
    /// The type being extracted.
    type Target;

    /// Retrives all the ocurrences of self in the source.
    fn all(&self, source: &Src) -> Vec<Self::Target>;
}
