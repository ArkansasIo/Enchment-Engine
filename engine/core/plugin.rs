//! Plugin: modular extension system.

pub trait Plugin {
    fn name(&self) -> &str;
    fn on_load(&mut self);
    fn on_unload(&mut self);
}
