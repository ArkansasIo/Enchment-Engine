//! Blueprint Graph: container for nodes and connections

use super::node::Node;

#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }
}
