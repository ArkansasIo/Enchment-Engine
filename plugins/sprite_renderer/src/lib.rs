pub struct SpriteRendererPlugin;
//! SpriteRenderer plugin: 2D sprite batching & sorting
use crate::super::plugin_manager::Plugin;

pub struct SpriteRendererPlugin;

impl Plugin for SpriteRendererPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize sprite renderer
    }
    fn name(&self) -> &'static str {
        "sprite_renderer"
    }
}
