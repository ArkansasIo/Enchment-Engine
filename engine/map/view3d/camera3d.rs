//! Camera3D: controls 3D map view.

pub struct Camera3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub zoom: f32,
}

impl Camera3D {
    pub fn new(x: f32, y: f32, z: f32, pitch: f32, yaw: f32, zoom: f32) -> Self {
        Self { x, y, z, pitch, yaw, zoom }
    }
}
