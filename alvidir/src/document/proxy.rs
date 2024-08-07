//! An access control layer for lazy loading and deferred persistence of documents.

use std::sync::Arc;

use async_std::sync::{RwLock, RwLockReadGuard};

use crate::{graph::Node, id::Identify};

use super::DocumentRepository;

/// Represents a [DocumentProxy] orchestrator.
pub trait ProxyTrigger {
    /// Returns true if, and only if, the document in the proxy has to be loaded from the
    /// repository.
    fn load(&self) -> bool;
}

/// An access control layer for a document from a [DocumentRepository] which is orchestrated by a
/// [ProxyTrigger].
pub struct DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    /// The repository of documents.
    document_repo: Arc<DocumentRepo>,
    /// The id of the document being cached.
    document_id: <DocumentRepo::Document as Identify>::Id,
    /// The cached state of the document.
    document: RwLock<DocumentRepo::Document>,
    /// The trigger that orchestrates the proxy.
    trigger: Trigger,
}

impl<DocumentRepo, Trigger> Identify for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    <DocumentRepo::Document as Identify>::Id: Clone,
{
    type Id = <DocumentRepo::Document as Identify>::Id;

    fn id(&self) -> Self::Id {
        self.document_id.clone()
    }
}

impl<DocumentRepo, Trigger> Node for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    DocumentRepo::Document: Node<Edge = <DocumentRepo::Document as Identify>::Id>,
    <DocumentRepo::Document as Identify>::Id: Clone,
    Trigger: ProxyTrigger,
{
    type Edge = <Self as Identify>::Id;

    async fn edges(&self) -> Vec<Self::Edge> {
        self.inner().await.edges().await
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    Trigger: ProxyTrigger,
{
    async fn inner(&self) -> RwLockReadGuard<DocumentRepo::Document> {
        if self.trigger.load() {
            self.update().await;
        }

        self.document.read().await
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    async fn update(&self) {
        if let Some(doc) = self.document_repo.find_by_id(&self.document_id).await {
            *self.document.write().await = doc;
        };
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    Trigger: Default,
{
    /// Returns a [DocumentProxy] constructor for a predefined [DocumentRepository] and
    /// [ProxyTrigger], requiring no more than the document to be provided.
    pub fn builder(document_repo: Arc<DocumentRepo>) -> impl Fn(DocumentRepo::Document) -> Self {
        move |document| -> Self {
            Self {
                document_repo: document_repo.clone(),
                trigger: Trigger::default(),
                document_id: document.id(),
                document: RwLock::new(document),
            }
        }
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::ProxyTrigger;

    /// A mock implementation for the [ProxyTrigger] trait.
    pub struct ProxyTriggerMock {
        pub load_fn: Option<fn() -> bool>,
    }

    impl ProxyTrigger for ProxyTriggerMock {
        fn load(&self) -> bool {
            self.load_fn.expect("load method must be set")()
        }
    }

    impl ProxyTriggerMock {
        pub fn with_load_fn(mut self, f: fn() -> bool) -> Self {
            self.load_fn = Some(f);
            self
        }
    }
}
