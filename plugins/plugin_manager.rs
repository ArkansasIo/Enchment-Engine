//! PluginManager: Loads, registers, and manages plugins at runtime
use std::collections::HashMap;

pub enum EngineEvent<'a> {
    Startup,
    Shutdown,
    FrameStart,
    FrameEnd,
    Custom(&'a str),
}

pub trait Plugin {
    fn register(&self);
    fn init(&self);
    fn name(&self) -> &'static str;
    fn on_event(&self, _event: &EngineEvent) {}
}

pub struct PluginManager {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: HashMap::new() }
    }
    pub fn load_plugin(&mut self, plugin: Box<dyn Plugin>) {
        let name = plugin.name().to_string();
        plugin.register();
        self.plugins.insert(name, plugin);
    }
    pub fn init_plugins(&self) {
        for plugin in self.plugins.values() {
            plugin.init();
        }
    }
    pub fn get_plugin(&self, name: &str) -> Option<&Box<dyn Plugin>> {
        self.plugins.get(name)
    }
    pub fn broadcast_event(&self, event: &EngineEvent) {
        for plugin in self.plugins.values() {
            plugin.on_event(event);
        }
    }
}
