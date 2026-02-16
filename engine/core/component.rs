//! Component system: attachable logic/data for Actors.

pub trait Component {
    fn update(&mut self, delta_time: f32);
}
