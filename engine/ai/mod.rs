// Artificial Intelligence module
pub mod behavior_tree;
pub mod blackboard;
pub mod eqs;
pub mod perception;
pub mod navigation;
pub mod crowd;
pub mod smart_object;
pub mod fsm;
pub mod planner;

// Example integration in engine:
// use crate::ai::fsm::*;
// use crate::ai::planner::*;
//
// fn ai_demo() {
//     // FSM Example
//     let mut fsm = fsm::Machine::new();
//     let idle = fsm::State::new("Idle");
//     let walk = fsm::State::new("Walk");
//     fsm.add_state(idle.clone());
//     fsm.add_state(walk.clone());
//     fsm.add_transition(fsm::Transition::new("Idle", "Walk", "start_walking"));
//     fsm.set_current("Idle");
//     fsm.update("start_walking");
//     assert_eq!(fsm.get_current(), Some("Walk"));
//
//     // Planner Example
//     let mut plan = planner::Plan::new();
//     plan.add_action(planner::Action::new("MoveTo"));
//     plan.execute();
// }
