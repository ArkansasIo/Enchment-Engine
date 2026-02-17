//! Blueprint Node: logic or data operation in a graph

use super::pin::Pin;

#[derive(Debug, Clone)]
pub struct Node {
    pub name: String,
    pub pins: Vec<Pin>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), pins: Vec::new() }
    }
    pub fn add_pin(&mut self, pin: Pin) {
        self.pins.push(pin);
    }
}
