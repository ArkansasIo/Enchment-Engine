pub struct SceneManagementPlugin;
//! SceneManagement plugin: scene loading & streaming
use crate::super::plugin_manager::Plugin;

pub struct SceneManagementPlugin;

impl Plugin for SceneManagementPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize scene management
    }
    fn name(&self) -> &'static str {
        "scene_management"
    }
}
