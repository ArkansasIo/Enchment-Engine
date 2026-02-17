//! Blueprint Function Graph: callable routine

use super::graph::Graph;

#[derive(Debug, Default)]
pub struct FunctionGraph {
    pub name: String,
    pub graph: Graph,
}

impl FunctionGraph {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), graph: Graph::new() }
    }
}
