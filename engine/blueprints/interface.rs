//! Blueprint Interface: implemented by Blueprints for communication

#[derive(Debug, Clone)]
pub struct Interface {
    pub name: String,
}

impl Interface {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
