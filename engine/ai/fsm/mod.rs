//! FSM module: re-exports for easy use and usage example.

mod state;
mod transition;
mod machine;

pub use state::State;
pub use transition::Transition;
pub use machine::Machine;

// Example usage:
// use crate::ai::fsm::*;
// let mut fsm = Machine::new();
// let idle = State::new("Idle");
// let walk = State::new("Walk");
// fsm.add_state(idle.clone());
// fsm.add_state(walk.clone());
// fsm.add_transition(Transition::new("Idle", "Walk", "start_walking"));
// fsm.set_current("Idle");
// fsm.update("start_walking");
// assert_eq!(fsm.get_current(), Some("Walk"));
