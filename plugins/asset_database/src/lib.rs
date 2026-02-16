pub struct AssetDatabasePlugin;
//! AssetDatabase plugin: asset management and import pipeline
use crate::super::plugin_manager::Plugin;

pub struct AssetDatabasePlugin;

impl Plugin for AssetDatabasePlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize asset database
    }
    fn name(&self) -> &'static str {
        "asset_database"
    }
}
