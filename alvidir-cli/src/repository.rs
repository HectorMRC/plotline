use std::{
    fs::File,
    path::{Path, PathBuf},
};

use alvidir::{document::DocumentRepository, id::Identify};

/// A file-system document.
pub struct Document {
    path: PathBuf,
    _file: File,
}

impl Identify for Document {
    type Id = PathBuf;

    fn id(&self) -> &Self::Id {
        &self.path
    }
}

/// Implements the [DocumentRepository] trait taking as datasource the given local directory.
pub struct LocalDocumentRepository<'a> {
    /// The base path in which the repository has to look up for files.
    pub context: &'a Path,
}

impl<'a> DocumentRepository for LocalDocumentRepository<'a> {
    type Document = Document;

    fn find_by_id(&self, _id: &<Self::Document as Identify>::Id) -> Option<Self::Document> {
        unimplemented!()
    }
}
