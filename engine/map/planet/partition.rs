//! WorldPartition: Unreal 5–style world partitioning and streaming for planet-scale maps.

use super::worldmap::WorldMap;

pub struct WorldPartition {
    pub region_size: u32,
    pub loaded_regions: Vec<(i32, i32)>, // (region_x, region_y)
}

impl WorldPartition {
    pub fn new(region_size: u32) -> Self {
        Self { region_size, loaded_regions: Vec::new() }
    }

    pub fn region_coords(&self, x: f32, y: f32) -> (i32, i32) {
        let rx = (x / self.region_size as f32).floor() as i32;
        let ry = (y / self.region_size as f32).floor() as i32;
        (rx, ry)
    }

    pub fn load_region(&mut self, region: (i32, i32)) {
        if !self.loaded_regions.contains(&region) {
            self.loaded_regions.push(region);
            // TODO: Load region data from disk or generate procedurally
        }
    }

    pub fn unload_region(&mut self, region: (i32, i32)) {
        self.loaded_regions.retain(|&r| r != region);
        // TODO: Unload region data and free resources
    }

    pub fn is_region_loaded(&self, region: (i32, i32)) -> bool {
        self.loaded_regions.contains(&region)
    }
}
