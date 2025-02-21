//! Document related definitions.

use crate::id::Identify;

pub mod lazy;

/// A repository in charge of document's persistance.
pub trait DocumentRepository {
    /// The type of document retrived by the repository.
    type Document: Identify;

    /// Retrives the document with the given id, if any.
    fn find_by_id(&self, id: &<Self::Document as Identify>::Id) -> Option<Self::Document>;
}
