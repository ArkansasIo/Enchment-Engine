//! CityMap: high-level procedural town/city structure

use super::voronoi::{Point, VoronoiCell, generate_sites, compute_voronoi};

#[derive(Debug, Clone)]
pub struct CityMap {
    pub cells: Vec<VoronoiCell>,
    pub size: u32,
}

impl CityMap {
    pub fn generate(seed: u64, size: u32) -> Self {
        let sites = generate_sites(seed, 32, size as f32);
        let cells = compute_voronoi(&sites, size as f32);
        Self { cells, size }
    }
}
