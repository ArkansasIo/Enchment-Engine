//! Custom place definition.

pub struct CustomPlace {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomPlace {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
