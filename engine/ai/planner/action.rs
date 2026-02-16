//! Action: represents an AI action.

pub struct Action {
    pub name: String,
    pub cost: f32,
}

impl Action {
    pub fn new(name: &str, cost: f32) -> Self {
        Self { name: name.to_string(), cost }
    }
}

//! Planner Action: represents an action in a plan.

#[derive(Clone, Debug)]
pub struct Action {
    pub name: String,
}

impl Action {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn execute(&self) {
        // Placeholder for action logic
        println!("Executing action: {}", self.name);
    }
}

// Example usage:
// let a = Action::new("MoveTo");
// a.execute();
