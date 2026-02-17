//! Voronoi diagram utilities for procedural map generation

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct VoronoiCell {
    pub site: Point,
    pub vertices: Vec<Point>,
}

pub fn generate_sites(seed: u64, count: usize, size: f32) -> Vec<Point> {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    (0..count)
        .map(|_| Point {
            x: rng.gen_range(0.0..size),
            y: rng.gen_range(0.0..size),
        })
        .collect()
}

// Placeholder for full Voronoi diagram generation
pub fn compute_voronoi(_sites: &[Point], _size: f32) -> Vec<VoronoiCell> {
    // TODO: Implement full Voronoi algorithm or use a crate
    vec![]
}
