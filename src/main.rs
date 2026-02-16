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

mod engine;

use engine::ai;
use engine::core;

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

    // Core system demo
    {
        use engine::core::demo::run_demo;
        run_demo();
    }

    // Map system demo
    {
        use engine::map::demo::run_map_demo;
        run_map_demo();
    }

    // AI demo
    ai_demo();
    core::logging::log("Engine started");
    core::input::process_input("KeyPress:W");
}

fn ai_demo() {
    // FSM Example
    let mut fsm = ai::fsm::Machine::new();
    let idle = ai::fsm::State::new("Idle");
    let walk = ai::fsm::State::new("Walk");
    fsm.add_state(idle.clone());
    fsm.add_state(walk.clone());
    fsm.add_transition(ai::fsm::Transition::new("Idle", "Walk", "start_walking"));
    fsm.set_current("Idle");
    println!("FSM current state: {:?}", fsm.get_current());
    fsm.update("start_walking");
    println!("FSM current state after update: {:?}", fsm.get_current());

    // Planner Example
    let mut plan = ai::planner::Plan::new();
    plan.add_action(ai::planner::Action::new("MoveTo"));
    plan.add_action(ai::planner::Action::new("Attack"));
    plan.execute();
}
