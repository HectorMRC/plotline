//! Identity definition.

/// Represents whatever entity it can be uniquely identified.
pub trait Identify {
    type Id;

    fn id(&self) -> &Self::Id;
}
