use std::hash::Hash;

/// Qualifies an entity of being uniquely identifiable.
pub trait Identify {
    type Id: Eq + Hash + Clone;

    fn id(&self) -> Self::Id;
}
