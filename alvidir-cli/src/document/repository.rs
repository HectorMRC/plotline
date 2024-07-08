use std::path::Path;

use alvidir::{
    document::Document,
    id::Identify,
};

use super::DocumentRepository;

/// Implements the [DocumentRepository] trait taking as datasource the given
/// local directory.
pub struct LocalDocumentRepository<'a> {
    /// The base path in which the repository has to look up for files.
    pub context: &'a Path,
}

impl<'a> DocumentRepository for LocalDocumentRepository<'a> {
    async fn find_by_id(&self, _id: <Document as Identify>::Id) -> anyhow::Result<Document> {
        todo!()     
    }
}
