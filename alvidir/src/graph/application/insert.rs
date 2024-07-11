use std::sync::{Arc, RwLock};

use crate::{graph::directed::DirectedGraph, id::Identify};

use super::GraphApplication;

/// Implements the insert node transaction.
pub struct InsertNode<T: Identify> {
    pub graph: Arc<RwLock<DirectedGraph<T>>>,
    pub node: Option<T>,
}

impl<T: Identify> InsertNode<T> {
    pub fn with_node(mut self, node: T) -> Self {
        self.node = Some(node);
        self
    }

    /// Executes the transaction.
    pub async fn execute(self) {
        unimplemented!()
    }
}

impl<T: Identify> GraphApplication<T> {
    pub fn insert_node(&self) -> InsertNode<T> {
        InsertNode { 
            graph: self.graph.clone(),
            node: Default::default()
        }
    }
}