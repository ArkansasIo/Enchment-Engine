//! Custom planet-level data.

pub struct CustomPlanetData {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomPlanetData {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
