//! Lazy document representation.

use std::{
    fmt::Debug,
    sync::{Arc, OnceLock},
};

use crate::{graph::Node, id::Identify, resource::Resource};

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

impl<DocumentRepo> Node for LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
    <DocumentRepo::Document as Identify>::Id: Resource<Source = DocumentRepo::Document> + Debug,
{
    type Edge = <Self as Identify>::Id;

    fn edges(&self) -> Vec<Self::Edge> {
        self.inner().map(Resource::all).unwrap_or_default()
    }
}

impl<DocumentRepo> LazyDocument<DocumentRepo>
where
    DocumentRepo: DocumentRepository,
    <DocumentRepo::Document as Identify>::Id: Debug,
{
    fn inner(&self) -> Option<&DocumentRepo::Document> {
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

        self.inner()
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
