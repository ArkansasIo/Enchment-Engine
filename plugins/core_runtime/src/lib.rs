pub struct CoreRuntimePlugin;
//! CoreRuntime plugin: engine loop, memory, timing, threading
use crate::super::plugin_manager::Plugin;

pub struct CoreRuntimePlugin;

impl Plugin for CoreRuntimePlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize core runtime systems
    }
    fn name(&self) -> &'static str {
        "core_runtime"
    }
}
