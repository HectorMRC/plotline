use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::{graph::Node, id::Identify};

/// Represents the persistency layer for [Document]s.
#[trait_make::make]
pub trait DocumentRepository {
    /// The document type retrived by the repository.
    type Document: Identify + Node<Edge = <Self::Document as Identify>::Id>;

    /// Retrives the [Document] with the given id.
    async fn find_by_id(&self, id: <Self::Document as Identify>::Id) -> Option<Self::Document>;
}

/// Represents a [DocumentProxy] orchestrator.
pub trait ProxyTrigger {
    /// Returns true if, and only if, the document in the proxy has to be updated from the
    /// repository.
    fn update(&self) -> bool;
}

/// A control access layer for a [Document] from a [DocumentRepository] which is orchestrated by a
/// [ProxyTrigger].
pub struct DocumentProxy<DocumentRepo, Trigger>
where 
    DocumentRepo: DocumentRepository
{
    /// The repository of documents.
    document_repo: Arc<DocumentRepo>,
    /// The cached state of the document.
    document: RwLock<DocumentRepo::Document>,
    /// The trigger that orchestrates the proxy.
    trigger: Trigger,
}

impl<DocumentRepo, Trigger> Identify for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    type Id = <DocumentRepo::Document as Identify>::Id;

    fn id(&self) -> Self::Id {
        match self.document.read() {
            Ok(document) => document.id(),
            Err(poison) => poison.get_ref().id(),
        }
    }
}

impl<DocumentRepo, Trigger> Node for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
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
        if self.trigger.update() {
            self.update().await;
        }

        match self.document.read() {
            Ok(doc_guard) => doc_guard,
            Err(poison) => poison.into_inner(),
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

        if let Some(doc) = self.document_repo.find_by_id(doc_guard.id()).await {
            *doc_guard = doc;
        }
    }
}

impl<DocumentRepo, Trigger> DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
    Trigger: Default,
{
    /// Returns a [DocumentProxy] constructor for a predefined [DocumentRepository] and
    /// [ProxyTrigger], requiring no more than the [Document] to be provided.
    pub fn builder(document_repo: Arc<DocumentRepo>) -> impl Fn(DocumentRepo::Document) -> Self {
        move |document| -> Self {
            Self {
                document_repo: document_repo.clone(),
                trigger: Trigger::default(),
                document: RwLock::new(document),
            }
        }
    }
}

#[cfg(any(test, feature = "fixtures"))]
pub mod fixtures {
    use super::ProxyTrigger;

    pub struct FakeTrigger(pub bool);

    impl ProxyTrigger for FakeTrigger {
        fn update(&self) -> bool {
            self.0
        }
    }
}
