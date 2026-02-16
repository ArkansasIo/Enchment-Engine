//! Zone: a region or biome in the map.

pub struct Zone {
    pub name: String,
    pub grid: Grid,
}

use super::grid::Grid;

impl Zone {
    pub fn new(name: &str, grid: Grid) -> Self {
        Self { name: name.to_string(), grid }
    }
}
