//! Mountain biome definition.

pub struct MountainBiome {
    pub peak_height: f32,
    pub snow_cap: bool,
}

impl MountainBiome {
    pub fn new(peak_height: f32, snow_cap: bool) -> Self {
        Self { peak_height, snow_cap }
    }
}
