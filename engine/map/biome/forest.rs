//! Forest biome definition.

pub struct ForestBiome {
    pub density: f32,
    pub tree_type: String,
}

impl ForestBiome {
    pub fn new(density: f32, tree_type: &str) -> Self {
        Self { density, tree_type: tree_type.to_string() }
    }
}
