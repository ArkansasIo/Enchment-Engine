//! Generic item definition


#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ItemType {
    Weapon,
    Armor,
    Consumable,
    Material,
    Quest,
    Tool,
    Trinket,
    WondrousItem,
    Spell,
    Scroll,
    Potion,
    Ring,
    Amulet,
    WorldObject,
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: Option<String>,
    pub attunement: Option<bool>,
    pub magic: Option<bool>,
    pub description: Option<String>,
    pub details: Option<String>,
    pub class_restriction: Option<Vec<String>>,
    pub race_restriction: Option<Vec<String>>,
    pub background_restriction: Option<Vec<String>>,
    pub proficiency_required: Option<Vec<String>>,
    pub stats: Option<ItemStats>,
    pub price: Option<u32>,
    pub stack_max: Option<u32>,
    pub features: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
    pub spell_slots: Option<u8>,
    pub charges: Option<u32>,
    pub uses: Option<u32>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, Default)]
pub struct ItemStats {
    pub str_mod: Option<i32>,
    pub dex_mod: Option<i32>,
    pub con_mod: Option<i32>,
    pub int_mod: Option<i32>,
    pub wis_mod: Option<i32>,
    pub cha_mod: Option<i32>,
    pub ac: Option<i32>,
    pub hp: Option<i32>,
    pub speed: Option<i32>,
    pub initiative: Option<i32>,
    pub saving_throws: Option<Vec<String>>,
    pub skill_bonuses: Option<Vec<String>>,
    pub resistances: Option<Vec<String>>,
    pub vulnerabilities: Option<Vec<String>>,
    pub immunities: Option<Vec<String>>,
}
