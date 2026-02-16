//! Landmark place definition.

pub struct Landmark {
    pub name: String,
    pub description: String,
}

impl Landmark {
    pub fn new(name: &str, description: &str) -> Self {
        Self { name: name.to_string(), description: description.to_string() }
    }
}
