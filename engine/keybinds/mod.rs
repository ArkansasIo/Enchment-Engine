//! Keybinds system: manage and query key bindings.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Keybinds {
    pub bindings: HashMap<String, String>, // action -> key
}

impl Keybinds {
    pub fn new() -> Self {
        Self { bindings: HashMap::new() }
    }
    pub fn bind(&mut self, action: &str, key: &str) {
        self.bindings.insert(action.to_string(), key.to_string());
    }
    pub fn get_key(&self, action: &str) -> Option<&str> {
        self.bindings.get(action).map(|s| s.as_str())
    }
    pub fn unbind(&mut self, action: &str) {
        self.bindings.remove(action);
    }
}

// Example usage:
// let mut kb = Keybinds::new();
// kb.bind("Jump", "Space");
// assert_eq!(kb.get_key("Jump"), Some("Space"));
