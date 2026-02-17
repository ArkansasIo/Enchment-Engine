//! Blueprint Macro Graph: inline-expanded logic

use super::graph::Graph;

#[derive(Debug, Default)]
pub struct MacroGraph {
    pub name: String,
    pub graph: Graph,
}

impl MacroGraph {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), graph: Graph::new() }
    }
}
