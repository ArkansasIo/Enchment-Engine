//! Village place definition.

pub struct Village {
    pub name: String,
    pub population: u32,
}

impl Village {
    pub fn new(name: &str, population: u32) -> Self {
        Self { name: name.to_string(), population }
    }
}
