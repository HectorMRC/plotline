use std::{
    fs::File,
    path::{Path, PathBuf},
    sync::Arc,
};

use alvidir::{
    document::{lazy::LazyDocument, DocumentRepository},
    id::Identify,
};
use ignore::Walk;
use regex::Regex;

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

/// Implements the [`DocumentRepository`] trait taking as datasource the given local directory.
pub struct LocalDocumentRepository<'a> {
    /// The base path in which the repository has to look up for files.
    pub context: &'a Path,
    /// The filename pattern.
    pub pattern: Regex,
}

impl<'a> DocumentRepository for LocalDocumentRepository<'a> {
    type Document = Document;

    fn find_by_id(&self, _id: &<Self::Document as Identify>::Id) -> Option<Self::Document> {
        unimplemented!()
    }
}

impl<'a> LocalDocumentRepository<'a> {
    pub fn all(self: &Arc<Self>) -> impl Iterator<Item = LazyDocument<Self>> + '_ {
        Walk::new(&self.context)
            .filter_map(move |entry| {
                if let Err(err) = &entry {
                    tracing::error!(
                        error = err.to_string(),
                        context = self.context.to_string_lossy().to_string(),
                        "walking base directory"
                    );
                }

                entry.ok()
            })
            .filter(move |entry| {
                let matches = self.pattern.is_match(&entry.file_name().to_string_lossy());
                tracing::debug!(path = entry.path().to_string_lossy().to_string(), matches);

                matches
            })
            .filter_map(move |entry| {
                let path = entry
                    .path()
                    .strip_prefix(&self.context)
                    .map(ToOwned::to_owned);

                if let Err(err) = &path {
                    tracing::error!(
                        error = err.to_string(),
                        path = entry.path().to_string_lossy().to_string(),
                        context = self.context.to_string_lossy().to_string(),
                        "stripping context from path"
                    );
                }

                path.ok()
            })
            .map(LazyDocument::builder(self.clone()))
    }
}
