//! Material Graph: node-based material logic

use super::graph::Graph;

#[derive(Debug, Default)]
pub struct MaterialGraph {
    pub graph: Graph,
}

impl MaterialGraph {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }
}
