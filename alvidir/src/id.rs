/// Qualifies an struct of being uniquely identifiable.
pub trait Identify {
    type Id: Eq;

    fn id(&self) -> &Self::Id;
}
