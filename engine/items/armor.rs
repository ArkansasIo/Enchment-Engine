//! Armor definition

use super::item::Item;


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ArmorType {
    Light,
    Medium,
    Heavy,
    Shield,
    Helmet,
    Chest,
    Legs,
    Boots,
    Gloves,
    Cloak,
    Accessory,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct Armor {
    pub base: Item,
    pub armor_type: ArmorType,
    pub ac: Option<i32>,
    pub stealth_disadvantage: Option<bool>,
    pub weight: Option<f32>,
    pub magic_bonus: Option<i32>,
    pub resistances: Option<Vec<String>>,
    pub vulnerabilities: Option<Vec<String>>,
    pub immunities: Option<Vec<String>>,
    pub stat_bonus: Option<String>,
    pub buff: Option<String>,
    pub debuff: Option<String>,
    pub special: Option<String>,
}
