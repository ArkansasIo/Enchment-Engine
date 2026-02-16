//! Desert biome definition.

pub struct DesertBiome {
    pub sand_type: String,
    pub oasis_count: u32,
}

impl DesertBiome {
    pub fn new(sand_type: &str, oasis_count: u32) -> Self {
        Self { sand_type: sand_type.to_string(), oasis_count }
    }
}
