pub struct Physics2DPlugin;
//! Physics2D plugin: 2D collision & rigidbody simulation
use crate::super::plugin_manager::Plugin;

pub struct Physics2DPlugin;

impl Plugin for Physics2DPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize 2D physics
    }
    fn name(&self) -> &'static str {
        "physics2d"
    }
}
