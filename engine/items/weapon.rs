//! Weapon definition

use super::item::Item;


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum WeaponType {
    SimpleMelee,
    SimpleRanged,
    MartialMelee,
    MartialRanged,
    Sword,
    Axe,
    Bow,
    Staff,
    Dagger,
    Mace,
    Gun,
    Wand,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct Weapon {
    pub base: Item,
    pub weapon_type: WeaponType,
    pub damage_dice: Option<String>, // e.g. "1d8"
    pub damage_type: Option<String>, // e.g. "slashing", "fire"
    pub properties: Option<Vec<String>>, // e.g. "finesse", "light", "two-handed"
    pub range: Option<String>, // e.g. "80/320"
    pub proficiency: Option<Vec<String>>,
    pub stat_bonus: Option<String>,
    pub buff: Option<String>,
    pub debuff: Option<String>,
    pub magic_bonus: Option<i32>,
    pub crit_range: Option<String>,
    pub ammo_type: Option<String>,
    pub reload: Option<u32>,
    pub special: Option<String>,
}
