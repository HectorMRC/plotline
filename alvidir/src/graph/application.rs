use crate::id::Identify;

use super::{directed::DirectedGraph, Node, Result};

pub struct GraphApplication<Node: Identify> {
    pub graph: DirectedGraph<Node>
}

impl<T: Node + Identify> GraphApplication<T> {
    pub async fn check(&self, id: T::Id) -> Result<()> { 
        // let node = self.graph.node(id);
        // let neighbor = node.references().await.remove(0);
        // node.references().await;
        Ok(())
    }
}