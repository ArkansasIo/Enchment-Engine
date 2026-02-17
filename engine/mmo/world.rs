//! MMO World: zones, events, persistence

#[derive(Debug, Clone, Default)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub players: Vec<u64>,
    pub npcs: Vec<u64>,
    pub events: Vec<WorldEvent>,
}

#[derive(Debug, Clone, Default)]
pub struct WorldEvent {
    pub id: String,
    pub description: String,
    pub active: bool,
}
