pub struct InputSystemPlugin;
//! InputSystem plugin: keyboard, mouse, gamepad, touch
use crate::super::plugin_manager::Plugin;

pub struct InputSystemPlugin;

impl Plugin for InputSystemPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize input system
    }
    fn name(&self) -> &'static str {
        "input_system"
    }
}
