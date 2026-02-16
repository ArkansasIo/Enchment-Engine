//! Subsystem: reusable engine service.

pub trait Subsystem {
    fn initialize(&mut self);
    fn shutdown(&mut self);
}
