//! Tundra biome definition.

pub struct TundraBiome {
    pub temperature: f32,
    pub permafrost: bool,
}

impl TundraBiome {
    pub fn new(temperature: f32, permafrost: bool) -> Self {
        Self { temperature, permafrost }
    }
}
