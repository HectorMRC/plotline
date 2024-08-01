use std::sync::{Arc, RwLock};

use crate::id::Identify;

use super::directed::DirectedGraph;

mod insert;
pub use insert::*;

pub struct GraphApplication<T: Identify> {
    pub graph: Arc<RwLock<DirectedGraph<T>>>,
}

impl<T: Identify> GraphApplication<T> {
    pub fn new(graph: DirectedGraph<T>) -> Self {
        Self { graph: Arc::new(RwLock::new(graph)) }
    }
}