//! Story root: contains acts

#[derive(Debug, Clone, Default)]
pub struct Story {
    pub id: String,
    pub title: String,
    pub description: String,
    pub acts: Vec<Act>,
}

use super::act::Act;
