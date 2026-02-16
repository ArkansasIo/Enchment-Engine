//! Camera2D: controls 2D map view.

pub struct Camera2D {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
}

impl Camera2D {
    pub fn new(x: f32, y: f32, zoom: f32) -> Self {
        Self { x, y, zoom }
    }
}
