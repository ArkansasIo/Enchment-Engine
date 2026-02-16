pub struct RendererCorePlugin;
//! RendererCore plugin: render pipeline framework
use crate::super::plugin_manager::Plugin;

pub struct RendererCorePlugin;

impl Plugin for RendererCorePlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize renderer core
    }
    fn name(&self) -> &'static str {
        "renderer_core"
    }
}
