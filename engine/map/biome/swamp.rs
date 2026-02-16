//! Swamp biome definition.

pub struct SwampBiome {
    pub water_level: f32,
    pub fog_density: f32,
}

impl SwampBiome {
    pub fn new(water_level: f32, fog_density: f32) -> Self {
        Self { water_level, fog_density }
    }
}
