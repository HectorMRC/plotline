//! Definitions for creating and managing arbitrary documents.

use crate::id::Identify;

pub mod proxy;

/// Represents the persistency layer for documents.
#[trait_make::make]
pub trait DocumentRepository {
    /// The document type retrived by the repository.
    type Document: Identify;

    /// Retrives the document with the given id.
    async fn find_by_id(&self, id: &<Self::Document as Identify>::Id) -> Option<Self::Document>;
}
