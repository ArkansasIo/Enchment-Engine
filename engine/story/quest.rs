//! Main Quest

#[derive(Debug, Clone, Default)]
pub struct Quest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub objectives: Vec<String>,
}
