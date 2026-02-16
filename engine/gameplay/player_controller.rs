//! PlayerController: handles player input and logic.

pub struct PlayerController {
    pub id: u64,
}

impl PlayerController {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
    pub fn process_input(&mut self) {
        // TODO: implement input handling
    }
}
