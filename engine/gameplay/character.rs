//! Character: advanced pawn with movement/animation.

pub struct Character {
    pub id: u64,
    pub name: String,
}

impl Character {
    pub fn new(id: u64, name: &str) -> Self {
        Self { id, name: name.to_string() }
    }
}
