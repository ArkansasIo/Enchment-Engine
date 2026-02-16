//! FSM Transition: represents a transition between states.

#[derive(Clone, Debug)]
pub struct Transition {
    pub from: String,
    pub to: String,
    pub condition: String,
}

impl Transition {
    pub fn new(from: &str, to: &str, condition: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            condition: condition.to_string(),
        }
    }
}

// Example usage:
// let t = Transition::new("Idle", "Walk", "start_walking");
