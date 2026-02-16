//! Main engine entry point: plugin registration example
mod plugins;
use plugins::plugin_manager::{PluginManager, Plugin, EngineEvent};

use plugins::core_runtime::CoreRuntimePlugin;
use plugins::renderer_core::RendererCorePlugin;
use plugins::sprite_renderer::SpriteRendererPlugin;
use plugins::mesh_renderer::MeshRendererPlugin;
use plugins::input_system::InputSystemPlugin;
use plugins::scene_management::SceneManagementPlugin;
use plugins::physics2d::Physics2DPlugin;
use plugins::asset_database::AssetDatabasePlugin;
use plugins::editor_viewport::EditorViewportPlugin;
use plugins::inspector_hierarchy::InspectorHierarchyPlugin;

fn main() {
    let mut plugin_manager = PluginManager::new();
    // Register core plugins
    plugin_manager.load_plugin(Box::new(CoreRuntimePlugin));
    plugin_manager.load_plugin(Box::new(RendererCorePlugin));
    plugin_manager.load_plugin(Box::new(SpriteRendererPlugin));
    plugin_manager.load_plugin(Box::new(MeshRendererPlugin));
    plugin_manager.load_plugin(Box::new(InputSystemPlugin));
    plugin_manager.load_plugin(Box::new(SceneManagementPlugin));
    plugin_manager.load_plugin(Box::new(Physics2DPlugin));
    plugin_manager.load_plugin(Box::new(AssetDatabasePlugin));
    plugin_manager.load_plugin(Box::new(EditorViewportPlugin));
    plugin_manager.load_plugin(Box::new(InspectorHierarchyPlugin));
    // Initialize all plugins
    plugin_manager.init_plugins();
    println!("All plugins registered and initialized.");

    // Engine lifecycle events
    plugin_manager.broadcast_event(&EngineEvent::Startup);
    for _frame in 0..3 {
        plugin_manager.broadcast_event(&EngineEvent::FrameStart);
        // ...engine/game logic...
        plugin_manager.broadcast_event(&EngineEvent::FrameEnd);
    }
    plugin_manager.broadcast_event(&EngineEvent::Shutdown);
}
