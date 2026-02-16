//! Custom building definition.

pub struct CustomBuilding {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomBuilding {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
