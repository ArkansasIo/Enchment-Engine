//! Blueprint Timeline: curve-driven update logic

#[derive(Debug, Clone)]
pub struct Timeline {
    pub name: String,
}

impl Timeline {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
