//! World object definition (non-equipment, e.g. chests, doors, etc.)

use super::item::Item;


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WorldObjectType {
    Chest,
    Door,
    Key,
    ResourceNode,
    Trap,
    Portal,
    Puzzle,
    Shrine,
    Campfire,
    Altar,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct WorldObject {
    pub base: Item,
    pub object_type: WorldObjectType,
    pub interactable: bool,
    pub locked: Option<bool>,
    pub trapped: Option<bool>,
    pub loot_table: Option<String>,
    pub script: Option<String>,
    pub details: Option<String>,
    pub on_interact: Option<String>,
}
