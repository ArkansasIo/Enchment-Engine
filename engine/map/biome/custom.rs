//! Custom biome definition.

pub struct CustomBiome {
    pub name: String,
    pub properties: Vec<(String, String)>,
}

impl CustomBiome {
    pub fn new(name: &str, properties: Vec<(String, String)>) -> Self {
        Self { name: name.to_string(), properties }
    }
}
