//! Grid2D: 2D grid overlay for maps.

pub struct Grid2D {
    pub width: u32,
    pub height: u32,
}

impl Grid2D {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
