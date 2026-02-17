//! Blueprint Construction Script: runs on construction/spawn

use super::graph::Graph;

#[derive(Debug, Default)]
pub struct ConstructionScript {
    pub graph: Graph,
}

impl ConstructionScript {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }
}
