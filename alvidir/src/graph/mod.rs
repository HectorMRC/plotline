//! Graph related definitions.

use std::collections::BTreeMap;

use crate::id::Identify;

mod proxy;
pub use proxy::*;

/// An arbitrary graph.
#[derive(Debug)]
pub struct Graph<T: Identify> {
    /// All the nodes in the graph.
    nodes: BTreeMap<T::Id, T>,
}

impl<T: Identify> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: Default::default(),
        }
    }
}

impl<T> FromIterator<T> for Graph<T>
where
    T: Identify,
    T::Id: Ord + Clone,
{
    /// Returns the [`Graph`] resulting from all the nodes in the given iterator.
    ///
    /// This method does not check if there are repeated ids. In front of collisions
    /// only the latest node will remain.
    fn from_iter<V: IntoIterator<Item = T>>(nodes: V) -> Self {
        Self {
            nodes: BTreeMap::from_iter(nodes.into_iter().map(|node| (node.id().clone(), node))),
        }
    }
}

impl<T> Graph<T>
where
    T: Identify,
    T::Id: Ord + Clone,
{
    /// Inserts the given node into the graph, overwriting any previous value with the same id.
    pub fn with_node(mut self, node: T) -> Self {
        self.nodes.insert(node.id().clone(), node);
        self
    }
}

impl<T: Identify> Graph<T> {
    /// Returns the [`NodeProxy`] for the given id.
    pub fn node(&self, id: T::Id) -> NodeProxy<'_, T> {
        NodeProxy { graph: self, id }
    }
}

// #[cfg(any(test, feature = "fixtures"))]
// pub mod fixtures {
//     use crate::id::Identify;

//     /// A fake node implementation.
//     #[derive(Debug, Default)]
//     pub struct NodeMock<Id: 'static> {
//         pub id_fn: Option<fn() -> &'static Id>,
//         pub edges_fn: Option<fn() -> Vec<Id>>,
//     }

//     impl<Id> Identify for NodeMock<Id> {
//         type Id = Id;

//         fn id(&self) -> &Self::Id {
//             self.id_fn.expect("id method must be set")()
//         }
//     }

//     impl<Id> NodeMock<Id> {
//         pub fn with_id_fn(mut self, f: fn() -> &'static Id) -> Self {
//             self.id_fn = Some(f);
//             self
//         }

//         pub fn with_edges_fn(mut self, f: fn() -> Vec<Id>) -> Self {
//             self.edges_fn = Some(f);
//             self
//         }
//     }

//     macro_rules! node_mock {
//         ($id:tt, $($edges:tt)*) => {
//             NodeMock::default()
//                 .with_id_fn(|| &$id)
//                 .with_edges_fn(|| vec![$($edges)*])
//         };
//     }

//     pub(crate) use node_mock;
// }
