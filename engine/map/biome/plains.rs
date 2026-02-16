//! Plains biome definition.

pub struct PlainsBiome {
    pub grass_height: f32,
    pub wildflowers: bool,
}

impl PlainsBiome {
    pub fn new(grass_height: f32, wildflowers: bool) -> Self {
        Self { grass_height, wildflowers }
    }
}
