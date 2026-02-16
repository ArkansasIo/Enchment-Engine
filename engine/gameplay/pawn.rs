//! Pawn: controllable entity base class.

pub struct Pawn {
    pub id: u64,
}

impl Pawn {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}
