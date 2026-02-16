//! Tick system: deterministic update loop.

pub trait Tickable {
    fn tick(&mut self, delta_time: f32);
}
