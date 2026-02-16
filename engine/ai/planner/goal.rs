//! Goal: represents an AI goal.

pub struct Goal {
    pub description: String,
}

impl Goal {
    pub fn new(description: &str) -> Self {
        Self { description: description.to_string() }
    }
}

//! Planner Goal: represents a goal for the planner.

#[derive(Clone, Debug)]
pub struct Goal {
    pub description: String,
}

impl Goal {
    pub fn new(description: &str) -> Self {
        Self { description: description.to_string() }
    }
}

// Example usage:
// let g = Goal::new("Find food");
