//! FSM State: represents a single state in the FSM.

#[derive(Clone, Debug)]
pub struct State {
    pub name: String,
}

impl State {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}

// Example usage:
// let idle = State::new("Idle");
// let walk = State::new("Walk");
