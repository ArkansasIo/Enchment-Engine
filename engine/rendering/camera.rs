//! Camera system: view and projection.

pub struct Camera {
    pub fov: f32,
}

impl Camera {
    pub fn new(fov: f32) -> Self {
        Self { fov }
    }
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
    }
}
