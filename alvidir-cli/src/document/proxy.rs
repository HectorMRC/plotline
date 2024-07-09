use std::sync::{Arc, RwLock, RwLockReadGuard};

use alvidir::{document::Document, graph::Node, id::Identify, property::Property, tag::Tag};
use tracing::error;

/// Represents the persistency layer between the proxy and the data-source.
#[trait_make::make]
pub trait DocumentRepository {
    /// Retrives the [Document] with the given id.
    async fn find_by_id(&self, id: <Document as Identify>::Id) -> anyhow::Result<Document>;
}

/// Represents an action triggerer.
pub trait ProxyTrigger {
    /// Returns true if, and only if, the document in the proxy has to be
    /// updated from the repository.
    fn update(&self) -> bool;
}

/// A control access layer for a [Document] from a [DocumentRepository] which
/// is orchestrated by a [ProxyTrigger].
pub struct DocumentProxy<DocumentRepo, Trigger> {
    /// The repository of documents.
    pub document_repo: Arc<DocumentRepo>,
    /// The trigger that orchestrates the proxy.
    pub trigger: Trigger,
    /// The cached state of the document.
    pub document: RwLock<Document>,
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
            Err(poison) => {
                error!(
                    error = "poisoned document",
                    document_id = poison.get_ref().id().to_string()
                );

                poison.into_inner()
            }
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
