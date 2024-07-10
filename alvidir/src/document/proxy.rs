use std::sync::{Arc, RwLock, RwLockReadGuard};

use crate::{document::Document, graph::Node, id::Identify, property::Property, tag::Tag};

/// Represents the persistency layer for [Document]s.
#[trait_make::make]
pub trait DocumentRepository {
    /// Retrives the [Document] with the given id.
    async fn find_by_id(&self, id: <Document as Identify>::Id) -> Option<Document>;
}

/// Represents a [DocumentProxy] orchestrator.
pub trait ProxyTrigger {
    /// Returns true if, and only if, the document in the proxy has to be
    /// updated from the repository.
    fn update(&self) -> bool;
}

/// A control access layer for a [Document] from a [DocumentRepository] which
/// is orchestrated by a [ProxyTrigger].
pub struct DocumentProxy<DocumentRepo, Trigger> {
    /// The repository of documents.
    document_repo: Arc<DocumentRepo>,
    /// The trigger that orchestrates the proxy.
    trigger: Trigger,
    /// The cached state of the document.
    document: RwLock<Document>,
}

impl<DocumentRepo, Trigger> Identify for DocumentProxy<DocumentRepo, Trigger>
where
    DocumentRepo: DocumentRepository,
{
    type Id = <Document as Identify>::Id;

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

    async fn tags(&self) -> Vec<Tag> {
        self.inner().await.tags().await
    }

    async fn properties(&self) -> Vec<Property<Self::Edge>> {
        self.inner().await.properties().await
    }

    async fn edges(&self) -> Vec<Self::Edge> {
        self.inner().await.edges().await
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
    Trigger: Default,
{
    /// Returns a [DocumentProxy] constructor for a predefined
    /// [DocumentRepository] and [ProxyTrigger], requiring no more than the
    /// [Document] to be provided.
    pub fn builder(document_repo: Arc<DocumentRepo>) -> impl Fn(Document) -> Self {
        move |document: Document| -> Self {
            Self {
                document_repo: document_repo.clone(),
                trigger: Trigger::default(),
                document: RwLock::new(document),
            }
        }
    }
}

#[cfg(any(test, features = "fixtures"))]
pub mod fixtures {
    use super::ProxyTrigger;

    pub struct FakeTrigger(pub bool);

    impl ProxyTrigger for FakeTrigger {
        fn update(&self) -> bool {
            self.0
        }
    }
}
