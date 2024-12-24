//! Resources from the schema.

use std::any::TypeId;

use crate::id::Identify;

use super::Schema;

/// A read-only access to a resource from the schema.
pub struct Read<'a, R> {
    _resource: Option<&'a R>,
}

impl<'a, T, R> From<&'a Schema<T>> for Read<'a, R>
where
    T: Identify,
    R: 'static,
{
    fn from(schema: &'a Schema<T>) -> Self {
        Self {
            _resource: schema
                .resources
                .get(&TypeId::of::<R>())
                .and_then(|res| res.downcast_ref()),
        }
    }
}
