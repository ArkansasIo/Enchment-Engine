//! Chaos Physics: main physics engine entry.

pub struct ChaosPhysics {
    pub enabled: bool,
}

impl ChaosPhysics {
    pub fn new() -> Self {
        Self { enabled: true }
    }
    pub fn simulate(&mut self, delta_time: f32) {
        // TODO: implement physics simulation
    }
}
