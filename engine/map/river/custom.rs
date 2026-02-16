//! Custom river feature.

pub struct CustomRiver {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomRiver {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
