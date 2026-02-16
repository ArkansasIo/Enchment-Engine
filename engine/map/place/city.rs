//! City place definition.

pub struct City {
    pub name: String,
    pub population: u32,
}

impl City {
    pub fn new(name: &str, population: u32) -> Self {
        Self { name: name.to_string(), population }
    }
}
