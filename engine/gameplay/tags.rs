//! Gameplay Tags: tag-based logic.

pub struct GameplayTag {
    pub name: String,
}

impl GameplayTag {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
