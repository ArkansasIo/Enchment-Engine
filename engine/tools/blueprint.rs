//! Blueprint: node-based visual scripting system.

pub struct Blueprint {
    pub name: String,
}

impl Blueprint {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn execute(&self) {
        // TODO: implement blueprint execution
    }
}
