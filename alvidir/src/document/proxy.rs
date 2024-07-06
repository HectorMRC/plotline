use std::sync::{RwLock, RwLockReadGuard};

use tracing::error;

use crate::{graph::DirectedGraphNode, id::Identify};

use super::{error::Result, Document, DocumentId};

#[trait_make::make]
pub trait DocumentRepository {
    /// Retrives the [Document] with the given id.
    async fn find_by_id(&self, id: DocumentId) -> Result<Document>;
}

pub trait ProxyTrigger {
    /// Returns true if, and only if, the document in the proxy has to be
    /// updated from the repository.
    fn update(&self) -> bool;
}

struct DocumentProxy<DocumentRepo, Trigger> {
    /// The repository of documents.
    pub document_repo: DocumentRepo,
    /// Triggers the document to update.
    pub trigger: Trigger,
    /// The cached state of the document.
    pub document: RwLock<Document>,
}

impl<DocumentRepo, Trigger> Identify for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    type Id = DocumentId;

    fn id(&self) -> Self::Id {
        match self.document.read() {
            Ok(document) => document.id(),
            Err(poison) => poison.get_ref().id(),
        }
    }
}

impl<DocumentRepo, Trigger> DirectedGraphNode for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    Trigger: ProxyTrigger,
{
    async fn tags(&self) -> Vec<crate::tag::Tag> {
        self.inner().await.tags().await
    }

    async fn properties(&self) -> Vec<crate::property::Property<Self::Id>> {
        self.inner().await.properties().await
    }

    async fn references(&self) -> Vec<Self::Id> {
        self.inner().await.references().await
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    Trigger: ProxyTrigger,
{
    async fn inner(&self) -> RwLockReadGuard<Document> {
        if self.trigger.update() {
            self.update().await;
        }

        match self.document.read() {
            Ok(doc_guard) => doc_guard,
            Err(poison) => {
                error!(
                    error = "poisoned document",
                    document_id = poison.get_ref().id().to_string()
                );

                poison.into_inner()
            },
        }
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    async fn update(&self) {
        let Ok(mut doc_guard) = self.document.write() else {
            return;
        };

        match self.document_repo.find_by_id(doc_guard.id()).await {
            Ok(document) => *doc_guard = document,
            Err(error) => error!(
                error = error.to_string(),
                document_id = doc_guard.id().to_string()
            ),
        };
    }
}