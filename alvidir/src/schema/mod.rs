/// Defines the structure and validation rules for data within the system.
pub trait Schema {
    /// The type over which the schema applies.
    type Document;
}
