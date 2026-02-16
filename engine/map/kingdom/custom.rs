//! Custom kingdom definition.

pub struct CustomKingdom {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomKingdom {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
