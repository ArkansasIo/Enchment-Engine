//! World: contains and manages all actors and systems.

use super::actor::Actor;

pub struct World {
    pub actors: Vec<Actor>,
}

impl World {
    pub fn new() -> Self {
        Self { actors: Vec::new() }
    }
    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }
    pub fn update(&mut self, delta_time: f32) {
        for actor in &mut self.actors {
            actor.update(delta_time);
        }
    }
}
