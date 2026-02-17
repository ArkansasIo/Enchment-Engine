//! Animation Graph (AnimBlueprint only)

use super::graph::Graph;

#[derive(Debug, Default)]
pub struct AnimationGraph {
    pub graph: Graph,
}

impl AnimationGraph {
    pub fn new() -> Self {
        Self { graph: Graph::new() }
    }
}
