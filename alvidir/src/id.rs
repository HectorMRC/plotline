//! Identity definition.

/// An entity that can be uniquely identified.
pub trait Identify {
    type Id;

    fn id(&self) -> &Self::Id;
}
