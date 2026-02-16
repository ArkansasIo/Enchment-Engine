//! Dungeon place definition.

pub struct DungeonPlace {
    pub name: String,
    pub depth: u32,
}

impl DungeonPlace {
    pub fn new(name: &str, depth: u32) -> Self {
        Self { name: name.to_string(), depth }
    }
}
