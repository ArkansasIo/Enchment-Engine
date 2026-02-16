//! Rigid Body Physics: basic physics objects.

pub struct RigidBody {
    pub mass: f32,
}

impl RigidBody {
    pub fn new(mass: f32) -> Self {
        Self { mass }
    }
    pub fn apply_force(&mut self, force: f32) {
        // TODO: implement force application
    }
}
