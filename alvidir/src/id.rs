//! Definition for qualifying entities of being identifiable.

/// Qualifies an entity of being uniquely identifiable.
pub trait Identify {
    type Id;

    fn id(&self) -> Self::Id;
}
