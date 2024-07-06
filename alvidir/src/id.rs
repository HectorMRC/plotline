use std::hash::Hash;

/// Qualifies an struct of being uniquely identifiable.
pub trait Identify {
    type Id: Eq + Hash;

    fn id(&self) -> Self::Id;
}
