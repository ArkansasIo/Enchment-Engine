// New features module for Eldiron

pub mod menus;
pub mod blueprint_system;

/// Launch the Blueprint Visual Scripting Editor (Unreal 5 style)
pub fn launch_blueprint_editor() {
    blueprint_system::launch_blueprint_editor();
}
pub fn example_feature() {
    println!("Example feature loaded and working!");
}
