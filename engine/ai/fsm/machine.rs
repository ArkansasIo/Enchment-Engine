//! FSM Machine: manages states and transitions.

use super::{state::State, transition::Transition};

pub struct Machine {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    pub current: Option<String>,
}

impl Machine {
    pub fn new() -> Self {
        Self { states: Vec::new(), transitions: Vec::new(), current: None }
    }
    pub fn add_state(&mut self, state: State) {
        self.states.push(state);
    }
    pub fn add_transition(&mut self, transition: Transition) {
        self.transitions.push(transition);
    }
    pub fn set_current(&mut self, name: &str) {
        self.current = Some(name.to_string());
    }
    pub fn update(&mut self, condition: &str) {
        if let Some(current) = &self.current {
            for t in &self.transitions {
                if &t.from == current && t.condition == condition {
                    self.current = Some(t.to.clone());
                    break;
                }
            }
        }
    }
    pub fn get_current(&self) -> Option<&str> {
        self.current.as_deref()
    }
}
