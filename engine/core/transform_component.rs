//! TransformComponent: position, rotation, scale for Actors.

use super::component::Component;

pub struct TransformComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl TransformComponent {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Component for TransformComponent {
    fn update(&mut self, _delta_time: f32) {
        // Example: could animate or move
    }
}
