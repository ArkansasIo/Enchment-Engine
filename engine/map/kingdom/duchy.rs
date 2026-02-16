//! Duchy kingdom definition.

pub struct Duchy {
    pub name: String,
    pub duke: String,
}

impl Duchy {
    pub fn new(name: &str, duke: &str) -> Self {
        Self { name: name.to_string(), duke: duke.to_string() }
    }
}
