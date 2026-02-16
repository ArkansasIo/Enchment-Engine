// Core engine module
pub mod actor;
pub mod component;
pub mod world;
pub mod subsystem;
pub mod plugin;
pub mod tick;
pub mod threading;
pub mod determinism;
pub mod build_pipeline;
pub mod transform_component;
pub mod demo;

/// Logging subsystem
pub mod logging {
    pub fn log(msg: &str) {
        println!("[LOG] {}", msg);
    }
}

/// Input subsystem
pub mod input {
    pub fn process_input(event: &str) {
        println!("Processing input: {}", event);
    }
}
