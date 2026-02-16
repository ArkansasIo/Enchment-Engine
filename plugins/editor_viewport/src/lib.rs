pub struct EditorViewportPlugin;
//! EditorViewport plugin: main editor viewport logic
use crate::super::plugin_manager::Plugin;

pub struct EditorViewportPlugin;

impl Plugin for EditorViewportPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize editor viewport
    }
    fn name(&self) -> &'static str {
        "editor_viewport"
    }
}
