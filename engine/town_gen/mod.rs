//! Procedural Town Generator (Rust port of MapForge)
// Formerly TownGeneratorOS

pub mod voronoi;
pub mod city_map;
pub mod building;
pub mod road;
pub mod ward;
pub mod image_export;

// Example entry point for generating a town
pub fn generate_town(seed: u64, size: u32) -> city_map::CityMap {
    city_map::CityMap::generate(seed, size)
}
