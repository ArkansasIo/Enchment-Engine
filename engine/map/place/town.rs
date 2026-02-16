//! Town place definition.

pub struct Town {
    pub name: String,
    pub population: u32,
}

impl Town {
    pub fn new(name: &str, population: u32) -> Self {
        Self { name: name.to_string(), population }
    }
}
