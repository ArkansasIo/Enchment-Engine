//! Macros system: manage and execute user-defined macros.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Macro {
    pub name: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Default)]
pub struct MacroSystem {
    pub macros: HashMap<String, Macro>,
}

impl MacroSystem {
    pub fn new() -> Self {
        Self { macros: HashMap::new() }
    }
    pub fn add_macro(&mut self, name: &str, actions: Vec<String>) {
        self.macros.insert(name.to_string(), Macro { name: name.to_string(), actions });
    }
    pub fn run_macro(&self, name: &str) {
        if let Some(mac) = self.macros.get(name) {
            for action in &mac.actions {
                println!("Running action: {}", action);
            }
        }
    }
}

// Example usage:
// let mut ms = MacroSystem::new();
// ms.add_macro("QuickSave", vec!["Save", "Notify"]); 
// ms.run_macro("QuickSave");
