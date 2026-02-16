//! State Machine: finite state logic.

pub struct StateMachine {
    pub state: String,
}

impl StateMachine {
    pub fn new(state: &str) -> Self {
        Self { state: state.to_string() }
    }
    pub fn transition(&mut self, new_state: &str) {
        self.state = new_state.to_string();
    }
}
