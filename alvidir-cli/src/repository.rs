use std::path::Path;

use alvidir::{
    document::{proxy::DocumentRepository, Document}, id::Identify
};

/// Implements the [DocumentRepository] trait taking as datasource the given
/// local directory.
pub struct LocalDocumentRepository<'a> {
    /// The base path in which the repository has to look up for files.
    pub context: &'a Path,
}

impl<'a> DocumentRepository for LocalDocumentRepository<'a> {
    async fn find_by_id(&self, _id: <Document as Identify>::Id) -> Option<Document> {
        unimplemented!() 
    }
}
