//! Planner: holds a sequence of actions to achieve a goal.

use super::action::Action;

#[derive(Debug, Default)]
pub struct Plan {
    pub actions: Vec<Action>,
}

impl Plan {
    pub fn new() -> Self {
        Self { actions: Vec::new() }
    }
    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }
    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }
    pub fn execute(&mut self) {
        for action in &self.actions {
            action.execute();
        }
    }
}

// Example usage:
// let mut plan = Plan::new();
// plan.add_action(Action::new("MoveTo"));
// plan.execute();
