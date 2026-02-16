//! Custom road feature.

pub struct CustomRoad {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomRoad {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
