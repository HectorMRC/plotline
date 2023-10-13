use std::sync::Arc;

/// An Entity is anything which to interact with.
pub struct Entity {
    /// An entity may be composed of other subentities
    subentities: Vec<Arc<Entity>>,
}
