use std::{fs, path::PathBuf, sync::Arc};

use plotline::{
    document::{lazy::LazyDocument, DocumentRepository},
    id::Identify,
};
use ignore::Walk;
use regex::Regex;

use crate::document::Document;

/// Implements the [`DocumentRepository`] trait taking as datasource the given local directory.
pub struct LocalDocumentRepository {
    /// The base path in which the repository has to look up for files.
    pub context: PathBuf,
    /// The file's extension.
    pub extension: String,
}

impl DocumentRepository for LocalDocumentRepository {
    type Document = Document;

    fn find_by_id(&self, id: &<Self::Document as Identify>::Id) -> Option<Self::Document> {
        let path = self.context.join(id).with_extension(&self.extension);

        fs::read(&path)
            .inspect_err(|err| {
                tracing::error!(
                    error = ?err,
                    id = ?id,
                    context = ?self.context,
                    path = ?path,
                    "finding document by id"
                )
            })
            .map(|bytes| Document { path, bytes })
            .ok()
    }
}

impl LocalDocumentRepository {
    /// Returns an iterator of [`LazyDocument`].
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
            .filter({
                let regex = Regex::new(&format!("\\.{}$", self.extension))
                    .expect("pattern should be a valid regular expression");

                move |entry| {
                    let matches = regex.is_match(&entry.file_name().to_string_lossy());
                    tracing::debug!(path = entry.path().to_string_lossy().to_string(), matches);

                    matches
                }
            })
            .filter_map(move |entry| {
                let path = entry
                    .path()
                    .with_extension("")
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
