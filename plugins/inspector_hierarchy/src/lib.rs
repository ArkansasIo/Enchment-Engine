pub struct InspectorHierarchyPlugin;
//! InspectorHierarchy plugin: inspector and hierarchy panels
use crate::super::plugin_manager::Plugin;

pub struct InspectorHierarchyPlugin;

impl Plugin for InspectorHierarchyPlugin {
    fn register(&self) {
        // Register with PluginManager
    }
    fn init(&self) {
        // Initialize inspector and hierarchy
    }
    fn name(&self) -> &'static str {
        "inspector_hierarchy"
    }
}
