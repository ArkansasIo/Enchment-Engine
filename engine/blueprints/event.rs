//! Blueprint Event: entry point node

use super::node::Node;

#[derive(Debug, Clone)]
pub struct Event {
    pub node: Node,
    pub event_type: String,
}

impl Event {
    pub fn new(event_type: &str) -> Self {
        Self {
            node: Node::new(event_type),
            event_type: event_type.to_string(),
        }
    }
}
