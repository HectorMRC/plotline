//! Lazy document representation.

use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use crate::{deref::TryDeref, id::Identify};

use super::DocumentRepository;

/// A lazy-loading document from a [`DocumentRepository`].
pub struct LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
{
    /// The repository of documents.
    document_repo: Arc<DocumentRepo>,
    /// The id of the document being cached.
    document_id: <DocumentRepo::Document as Identify>::Id,
    /// The cached state of the document.
    document: OnceLock<DocumentRepo::Document>,
}

impl<DocumentRepo> Identify for LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
{
    type Id = <DocumentRepo::Document as Identify>::Id;

    fn id(&self) -> &Self::Id {
        &self.document_id
    }
}

impl<DocumentRepo> TryDeref for LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
    <DocumentRepo::Document as Identify>::Id: Debug,
{
    type Target = DocumentRepo::Document;

    fn try_deref(&self) -> Option<&Self::Target> {
        if let doc @ Some(_) = self.document.get() {
            return doc;
        }

        if self
            .document
            .set(self.document_repo.find_by_id(&self.document_id)?)
            .is_err()
        {
            tracing::error!(id = format!("{:?}", self.document_id), "document not found");
            return None;
        }

        self.document.get()
    }
}

impl<DocumentRepo> LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
    <DocumentRepo::Document as Identify>::Id: Clone,
{
    /// Returns a [`LazyDocument`] constructor for a predefined [`DocumentRepository`].
    pub fn builder(
        document_repo: Arc<DocumentRepo>,
    ) -> impl Fn(<DocumentRepo::Document as Identify>::Id) -> Self {
        move |document_id| -> Self {
            Self {
                document_repo: document_repo.clone(),
                document_id,
                document: Default::default(),
            }
        }
    }
}

impl<DocumentRepo> LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
    DocumentRepo::Document: Debug,
    <DocumentRepo::Document as Identify>::Id: Clone,
{
    /// Returns a [`LazyDocument`] with the given repository and content.
    pub fn new(document_repo: Arc<DocumentRepo>, document: DocumentRepo::Document) -> Self {
        let lazy_doc = Self {
            document_repo: document_repo.clone(),
            document_id: document.id().clone(),
            document: Default::default(),
        };

        lazy_doc
            .document
            .set(document)
            .expect("default once-lock should be uninitialized");

        lazy_doc
    }
}
