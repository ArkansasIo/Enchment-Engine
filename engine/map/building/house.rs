//! House building definition.

pub struct House {
    pub owner: String,
    pub rooms: u32,
}

impl House {
    pub fn new(owner: &str, rooms: u32) -> Self {
        Self { owner: owner.to_string(), rooms }
    }
}
