//! Grid3D: 3D grid overlay for maps.

pub struct Grid3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl Grid3D {
    pub fn new(width: u32, height: u32, depth: u32) -> Self {
        Self { width, height, depth }
    }
}
