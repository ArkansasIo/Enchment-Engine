//! Behavior Tree: AI decision-making system.

pub struct BehaviorTree {
    pub name: String,
}

impl BehaviorTree {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn tick(&mut self) {
        // TODO: implement behavior tree tick
    }
}
