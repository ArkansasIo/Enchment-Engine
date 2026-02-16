//! Demo: simple integration test for core systems.

use super::{actor::Actor, transform_component::TransformComponent, world::World, component::Component};

pub fn run_demo() {
    let mut world = World::new();
    let mut actor = Actor::new(1, "Player");
    actor.add_component(Box::new(TransformComponent::new(0.0, 0.0, 0.0)));
    world.add_actor(actor);
    // Simulate update loop
    for _ in 0..3 {
        world.update(0.016); // ~60 FPS
    }
}
