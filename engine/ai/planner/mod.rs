//! Planner module: re-exports for easy use and usage example.

mod goal;
mod action;
mod plan;

pub use goal::Goal;
pub use action::Action;
pub use plan::Plan;

// Example usage:
// use crate::ai::planner::*;
// let mut plan = Plan::new();
// plan.add_action(Action::new("MoveTo"));
// plan.execute();
