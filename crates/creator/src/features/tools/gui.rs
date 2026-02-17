use crate::features::menus::{MenuItem, build_feature_menu};
#[derive(Clone, Debug)]
pub enum ItemType {
    Weapon,
    Armor,
    Accessory,
    Consumable,
    KeyItem,
    Material,
    Misc,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum WeaponType {
    Sword,
    Axe,
    Spear,
    Bow,
    Dagger,
    Staff,
    Wand,
    Fist,
    Gun,
    Whip,
    Katana,
    Hammer,
    Flail,
    Mace,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum ArmorType {
    Helmet,
    Chest,
    Gloves,
    Boots,
    Shield,
    Cloak,
    Accessory,
    Other(String),
}

#[derive(Clone)]
pub struct ItemEntry {
    pub name: String,
    pub item_type: ItemType,
    pub subtype: String,
    pub description: String,
    pub stats: Vec<(String, i32)>,
    pub rarity: String,
    pub value: i32,
    pub usable_in_battle: bool,
    pub usable_in_field: bool,
}

#[derive(Clone)]
pub struct WeaponEntry {
    pub name: String,
    pub weapon_type: WeaponType,
    pub description: String,
    pub stats: Vec<(String, i32)>,
    pub rarity: String,
    pub value: i32,
    pub usable_by: Vec<String>, // Classes/races
}

#[derive(Clone)]
pub struct ArmorEntry {
    pub name: String,
    pub armor_type: ArmorType,
    pub description: String,
    pub stats: Vec<(String, i32)>,
    pub rarity: String,
    pub value: i32,
    pub usable_by: Vec<String>,
}

#[derive(Clone)]
pub struct LootTableEntry {
    pub item: String,
    pub chance: f32,
    pub min_qty: u32,
    pub max_qty: u32,
}
#[derive(Clone, Debug)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Orc,
    Goblin,
    Dragonkin,
    Undead,
    Beast,
    Demon,
    Angel,
    Construct,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum CharacterClass {
    Warrior,
    Mage,
    Thief,
    Cleric,
    Ranger,
    Paladin,
    Monk,
    Bard,
    Necromancer,
    Summoner,
    Berserker,
    Knight,
    Samurai,
    Ninja,
    Alchemist,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum NpcType {
    Friendly,
    Enemy,
    Merchant,
    QuestGiver,
    Boss,
    Miniboss,
    Elite,
    Common,
    Other(String),
}

#[derive(Clone, Debug)]
pub enum Subtype {
    Fire,
    Ice,
    Lightning,
    Poison,
    Holy,
    Dark,
    Mechanical,
    Undead,
    Beast,
    Other(String),
}

// Example: List of available races, classes, types, subtypes for dropdowns or selection
pub const RACES: &[&str] = &["Human", "Elf", "Dwarf", "Orc", "Goblin", "Dragonkin", "Undead", "Beast", "Demon", "Angel", "Construct"];
pub const CLASSES: &[&str] = &["Warrior", "Mage", "Thief", "Cleric", "Ranger", "Paladin", "Monk", "Bard", "Necromancer", "Summoner", "Berserker", "Knight", "Samurai", "Ninja", "Alchemist"];
pub const NPC_TYPES: &[&str] = &["Friendly", "Enemy", "Merchant", "QuestGiver", "Boss", "Miniboss", "Elite", "Common"];
pub const SUBTYPES: &[&str] = &["Fire", "Ice", "Lightning", "Poison", "Holy", "Dark", "Mechanical", "Undead", "Beast"];
mod pixel_assets;
use pixel_assets::*;
mod items_menu;
use items_menu::ItemsMenuState;
//! GUI tools for MMORPG features and extensible toolbars.
//
// This module provides the main GUI struct and logic for all feature tools, toolbars, and editors.
// It is designed to be easily extensible for new tools, editors, and game features.
/// Loot table entry for the loot editor.
/// Skill tree entry for the skill tree editor.
/// Enemy or boss entry for the enemy/boss editor.
/// Story entry for the story editor.
/// Quest entry for the quest editor.
/// Act entry for the act editor.
/// Chapter entry for the chapter editor.
/// Reward entry for the rewards editor.
/// Simple entry for city, town, kingdom, etc. editors.

use egui::*;
use crate::features::tools::map_forge::MapForgeState;

mod blueprint_editor;
use blueprint_editor::BlueprintEditorGui;

/// Main GUI struct for all feature tools and editors.
///
/// This struct manages the state for all toolbars, editors, and feature dialogs.
/// Extend this struct to add new tools, editors, or feature states.
pub struct FeatureToolGui {
    /// Blueprint visual scripting editor
    show_blueprint_editor: bool,
    blueprint_editor: BlueprintEditorGui,
    /// Feature tool dialog states
    show_loot: bool,
    show_skill_tree: bool,
    show_enemy_boss: bool,
    show_spell: bool,
    show_level: bool,
    show_equipment: bool,
    show_inventory: bool,
    show_stat: bool,
    show_item_tier: bool,
    show_biome: bool,
    show_encounter: bool,
    show_group_finder: bool,
    show_world_boss: bool,
    show_event: bool,
    show_world_map: bool,
    show_story: bool,
    show_quest: bool,
    show_side_quest: bool,
    show_act: bool,
    show_chapter: bool,
    show_rewards: bool,
    show_city: bool,
    show_town: bool,
    show_kingdom: bool,
    show_underground: bool,
    show_zone: bool,
    show_world: bool,
    show_realm: bool,
    show_race: bool,
    show_class: bool,
    show_profession: bool,
    map_forge: MapForgeState,
    // New RPG/JRPG/MMORPG data collections
    pub all_races: Vec<String>,
    pub all_classes: Vec<String>,
    pub all_types: Vec<String>,
    pub all_subtypes: Vec<String>,
    pub all_items: Vec<ItemEntry>,
    pub all_weapons: Vec<WeaponEntry>,
    pub all_armors: Vec<ArmorEntry>,
    pub all_loot_tables: Vec<LootTableEntry>,
    pub all_npcs: Vec<NpcEntry>,
    pub all_enemies: Vec<EnemyEntry>,
    // Loot editor state
    loot_table: Vec<LootEntry>,
    loot_name: String,
    loot_chance: f32,
    loot_edit_idx: Option<usize>,
    // Skill tree editor state
    skill_tree: Vec<SkillEntry>,
    skill_name: String,
    skill_desc: String,
    skill_edit_idx: Option<usize>,
    // Enemy/Boss editor state
    enemy_list: Vec<EnemyEntry>,
    enemy_name: String,
    enemy_type: String,
    enemy_edit_idx: Option<usize>,
    // Story/Quest/Act/Reward editors
    story_list: Vec<StoryEntry>,
    story_title: String,
    story_edit_idx: Option<usize>,
    quest_list: Vec<QuestEntry>,
    quest_title: String,
    quest_edit_idx: Option<usize>,
    side_quest_list: Vec<QuestEntry>,
    side_quest_title: String,
    side_quest_edit_idx: Option<usize>,
    act_list: Vec<ActEntry>,
    act_title: String,
    act_edit_idx: Option<usize>,
    chapter_list: Vec<ChapterEntry>,
    chapter_title: String,
    chapter_edit_idx: Option<usize>,
    reward_list: Vec<RewardEntry>,
    reward_desc: String,
    reward_edit_idx: Option<usize>,
    // City/World/Zone editors
    city_list: Vec<SimpleEntry>,
    city_name: String,
    city_edit_idx: Option<usize>,
    town_list: Vec<SimpleEntry>,
    town_name: String,
    town_edit_idx: Option<usize>,
    kingdom_list: Vec<SimpleEntry>,
    kingdom_name: String,
    kingdom_edit_idx: Option<usize>,
    underground_list: Vec<SimpleEntry>,
    underground_name: String,
    underground_edit_idx: Option<usize>,
    zone_list: Vec<SimpleEntry>,
    zone_name: String,
    zone_edit_idx: Option<usize>,
    world_list: Vec<SimpleEntry>,
    world_name: String,
    world_edit_idx: Option<usize>,
    realm_list: Vec<SimpleEntry>,
    realm_name: String,
    realm_edit_idx: Option<usize>,
    // DnD 5e Races, Classes, Professions
    race_list: Vec<SimpleEntry>,
    race_name: String,
    race_edit_idx: Option<usize>,
    class_list: Vec<SimpleEntry>,
    class_name: String,
    class_edit_idx: Option<usize>,
    profession_list: Vec<SimpleEntry>,
    profession_name: String,
    profession_edit_idx: Option<usize>,
    // Items menu state
    item_name: String,
    item_desc: String,
    item_value: i32,
    item_edit_idx: Option<usize>,
}

#[derive(Clone)]
struct LootEntry {
    name: String,
    chance: f32,
}

#[derive(Clone)]
struct SkillEntry {
    name: String,
    desc: String,
}

#[derive(Clone)]
struct EnemyEntry {
    name: String,
    kind: String, // e.g. Boss, Mob, Elite, etc.
    race: String,
    enemy_type: String,
    subtype: String,
    class: String,
    subclass: String,
    dna: Option<NpcDna>,
    equipment: Vec<EquipmentEntry>,
    loot_table: Vec<LootEntry>,
}

#[derive(Clone)]
struct NpcEntry {
    name: String,
    race: String,
    npc_type: String,
    subtype: String,
    class: String,
    subclass: String,
    dna: Option<NpcDna>,
    equipment: Vec<EquipmentEntry>,
    loot_table: Vec<LootEntry>,
}

#[derive(Clone)]
struct NpcDna {
    strength: u8,
    agility: u8,
    intelligence: u8,
    vitality: u8,
    luck: u8,
    traits: Vec<String>,
}

#[derive(Clone)]
struct EquipmentEntry {
    name: String,
    kind: String, // Weapon, Armor, Accessory, etc.
    slot: String, // MainHand, OffHand, Head, Body, etc.
    stats: Vec<(String, i32)>,
    rarity: String,
}

#[derive(Clone)]
struct StoryEntry {
    title: String,
}

#[derive(Clone)]
struct QuestEntry {
    title: String,
}

#[derive(Clone)]
struct ActEntry {
    title: String,
}

#[derive(Clone)]
struct ChapterEntry {
    title: String,
}

#[derive(Clone)]
struct RewardEntry {
    desc: String,
}

#[derive(Clone)]
struct SimpleEntry {
    name: String,
}

pub struct FeatureToolGui {
    /// Blueprint visual scripting editor
    show_blueprint_editor: bool,
    blueprint_editor: BlueprintEditorGui,
    /// Feature tool dialog states
    show_loot: bool,
    show_skill_tree: bool,
    show_enemy_boss: bool,
    show_spell: bool,
    show_level: bool,
    show_equipment: bool,
    show_inventory: bool,
    show_stat: bool,
    show_item_tier: bool,
    show_biome: bool,
    show_encounter: bool,
    show_group_finder: bool,
    show_world_boss: bool,
    show_event: bool,
    show_story: bool,
    show_quest: bool,
    show_side_quest: bool,
    show_act: bool,
    show_chapter: bool,
    show_rewards: bool,
    show_city: bool,
    show_town: bool,
    show_kingdom: bool,
    show_underground: bool,
    show_zone: bool,
    show_world: bool,
    show_realm: bool,
    show_race: bool,
    show_class: bool,
    show_profession: bool,
    map_forge: MapForgeState,
    // New RPG/JRPG/MMORPG data collections
    pub all_races: Vec<String>,
    pub all_classes: Vec<String>,
    pub all_types: Vec<String>,
    pub all_subtypes: Vec<String>,
    pub all_items: Vec<ItemEntry>,
    pub all_weapons: Vec<WeaponEntry>,
    pub all_armors: Vec<ArmorEntry>,
    pub all_loot_tables: Vec<LootTableEntry>,
    pub all_npcs: Vec<NpcEntry>,
    pub all_enemies: Vec<EnemyEntry>,
    // Loot editor state
    loot_table: Vec<LootEntry>,
    loot_name: String,
    loot_chance: f32,
    loot_edit_idx: Option<usize>,
    // Skill tree editor state
    skill_tree: Vec<SkillEntry>,
    skill_name: String,
    skill_desc: String,
    skill_edit_idx: Option<usize>,
    // Enemy/Boss editor state
    enemy_list: Vec<EnemyEntry>,
    enemy_name: String,
    enemy_type: String,
    enemy_edit_idx: Option<usize>,
    // Story/Quest/Act/Reward editors
    story_list: Vec<StoryEntry>,
    story_title: String,
    story_edit_idx: Option<usize>,
    quest_list: Vec<QuestEntry>,
    quest_title: String,
    quest_edit_idx: Option<usize>,
    side_quest_list: Vec<QuestEntry>,
    side_quest_title: String,
    side_quest_edit_idx: Option<usize>,
    act_list: Vec<ActEntry>,
    act_title: String,
    act_edit_idx: Option<usize>,
    chapter_list: Vec<ChapterEntry>,
    chapter_title: String,
    chapter_edit_idx: Option<usize>,
    reward_list: Vec<RewardEntry>,
    reward_desc: String,
    reward_edit_idx: Option<usize>,
    // City/World/Zone editors
    city_list: Vec<SimpleEntry>,
    city_name: String,
    city_edit_idx: Option<usize>,
    town_list: Vec<SimpleEntry>,
    town_name: String,
    town_edit_idx: Option<usize>,
    kingdom_list: Vec<SimpleEntry>,
    kingdom_name: String,
    kingdom_edit_idx: Option<usize>,
    underground_list: Vec<SimpleEntry>,
    underground_name: String,
    underground_edit_idx: Option<usize>,
    zone_list: Vec<SimpleEntry>,
    zone_name: String,
    zone_edit_idx: Option<usize>,
    world_list: Vec<SimpleEntry>,
    world_name: String,
    world_edit_idx: Option<usize>,
    realm_list: Vec<SimpleEntry>,
    realm_name: String,
    realm_edit_idx: Option<usize>,
    // DnD 5e Races, Classes, Professions
    race_list: Vec<SimpleEntry>,
    race_name: String,
    race_edit_idx: Option<usize>,
    class_list: Vec<SimpleEntry>,
    class_name: String,
    class_edit_idx: Option<usize>,
    profession_list: Vec<SimpleEntry>,
    profession_name: String,
    profession_edit_idx: Option<usize>,
    // Items menu state
    item_name: String,
    item_desc: String,
    item_value: i32,
    item_edit_idx: Option<usize>,
}

#[derive(Clone)]
struct LootEntry {
    name: String,
    chance: f32,
}

#[derive(Clone)]
struct SkillEntry {
    name: String,
    desc: String,
}

#[derive(Clone)]
struct EnemyEntry {
    name: String,
    kind: String, // e.g. Boss, Mob, Elite, etc.
    race: String,
    enemy_type: String,
    subtype: String,
    class: String,
    subclass: String,
    dna: Option<NpcDna>,
    equipment: Vec<EquipmentEntry>,
    loot_table: Vec<LootEntry>,
}

#[derive(Clone)]
struct NpcEntry {
    name: String,
    race: String,
    npc_type: String,
    subtype: String,
    class: String,
    subclass: String,
    dna: Option<NpcDna>,
    equipment: Vec<EquipmentEntry>,
    loot_table: Vec<LootEntry>,
}

#[derive(Clone)]
struct NpcDna {
    strength: u8,
    agility: u8,
    intelligence: u8,
    vitality: u8,
    luck: u8,
    traits: Vec<String>,
}

#[derive(Clone)]
struct EquipmentEntry {
    name: String,
    kind: String, // Weapon, Armor, Accessory, etc.
    slot: String, // MainHand, OffHand, Head, Body, etc.
    stats: Vec<(String, i32)>,
    rarity: String,
}

#[derive(Clone)]
struct StoryEntry {
    title: String,
}

#[derive(Clone)]
struct QuestEntry {
    title: String,
}

#[derive(Clone)]
struct ActEntry {
    title: String,
}

#[derive(Clone)]
struct ChapterEntry {
    title: String,
}

#[derive(Clone)]
struct RewardEntry {
    desc: String,
}

#[derive(Clone)]
struct SimpleEntry {
    name: String,
}

impl FeatureToolGui {
        /// Recursively render a menu and handle submenu clicks.
        fn render_menu_recursive(&mut self, ui: &mut egui::Ui, menu: &MenuItem) {
            if menu.submenus.is_empty() {
                if ui.button(&menu.name).clicked() {
                    self.handle_menu_action(&menu.name);
                }
            } else {
                ui.menu_button(&menu.name, |ui| {
                    for submenu in &menu.submenus {
                        self.render_menu_recursive(ui, submenu);
                    }
                });
            }
        }

        /// Handle menu/submenu actions by name.
        fn handle_menu_action(&mut self, name: &str) {
            match name {
                // --- RPG Features ---
                "Quests" | "Main Quests" | "Side Quests" | "Quest Chains" | "Quest Rewards" => self.show_quest = true,
                "Inventory" | "Items" | "Equipment" | "Key Items" | "Consumables" => self.show_inventory = true,
                "Character Progression" | "Leveling" | "Skill Trees" | "Classes" | "Subclasses" | "Stats & Attributes" => self.show_level = true,
                "Turn-based Combat" | "Battle System" | "Enemy Groups" | "Boss Battles" | "Status Effects" => self.show_enemy_boss = true,
                "Loot Tables" => self.show_loot = true,

                // --- MMORPG Features ---
                "Guilds" | "Guild Creation" | "Guild Management" | "Guild Quests" | "Guild Wars" => self.show_group_finder = true,
                "Trading" | "Player Trading" | "Auction House" | "Marketplaces" => self.show_event = true,
                "PvP Arenas" | "1v1 Duels" | "Team Battles" | "Ranked Matches" => self.show_encounter = true,
                "World Events" | "Seasonal Events" | "Boss Raids" | "Server-wide Quests" => self.show_event = true,

                // --- Game Logic ---
                "Scripting" | "Visual Scripting" | "Lua Scripts" | "Event Triggers" => self.show_story = true,
                "AI Behaviors" | "Enemy AI" | "NPC Schedules" | "Pathfinding" => self.show_stat = true,
                "Dialogue Trees" | "Branching Dialogue" | "Voice Acting" | "Cinematic Events" => self.show_story = true,

                // --- Map Forge Systems ---
                "Map Editor"
                | "Tile Editor"
                | "Object Placer"
                | "Event Editor"
                | "Cutscene Editor"
                | "Paint Tool"
                | "Erase Tool"
                | "Fill Tool"
                | "Selection Tool"
                | "Object Placement Tool"
                | "Terrain Layer"
                | "Collision Layer"
                | "Object Layer"
                | "Event Layer"
                | "Toggle Grid"
                | "Snap To Grid"
                | "Resize Map"
                | "Export Map" => {
                    self.map_forge.open();
                }

                _ => {},
            }
        }
    pub fn new() -> Self {
        Self {
            show_blueprint_editor: false,
            blueprint_editor: BlueprintEditorGui::new(),
            show_loot: false,
            show_skill_tree: false,
            show_enemy_boss: false,
            show_spell: false,
            show_level: false,
            show_equipment: false,
            show_inventory: false,
            show_stat: false,
            show_item_tier: false,
            show_biome: false,
            show_encounter: false,
            show_group_finder: false,
            show_world_boss: false,
            show_event: false,
            show_story: false,
            show_quest: false,
            show_side_quest: false,
            show_act: false,
            show_chapter: false,
            show_rewards: false,
            loot_table: Vec::new(),
            loot_name: String::new(),
            loot_chance: 1.0,
            loot_edit_idx: None,
            skill_tree: Vec::new(),
            skill_name: String::new(),
            skill_desc: String::new(),
            skill_edit_idx: None,
            enemy_list: Vec::new(),
            enemy_name: String::new(),
            enemy_type: String::new(),
            enemy_edit_idx: None,
            story_list: Vec::new(),
            story_title: String::new(),
            story_edit_idx: None,
            quest_list: Vec::new(),
            quest_title: String::new(),
            quest_edit_idx: None,
            side_quest_list: Vec::new(),
            side_quest_title: String::new(),
            side_quest_edit_idx: None,
            act_list: Vec::new(),
            act_title: String::new(),
            act_edit_idx: None,
            chapter_list: Vec::new(),
            chapter_title: String::new(),
            chapter_edit_idx: None,
            reward_list: Vec::new(),
            reward_desc: String::new(),
            reward_edit_idx: None,
            show_city: false,
            show_town: false,
            show_kingdom: false,
            show_underground: false,
            show_zone: false,
            show_world: false,
            show_realm: false,
            city_list: Vec::new(),
            city_name: String::new(),
            city_edit_idx: None,
            town_list: Vec::new(),
            town_name: String::new(),
            town_edit_idx: None,
            kingdom_list: Vec::new(),
            kingdom_name: String::new(),
            kingdom_edit_idx: None,
            underground_list: Vec::new(),
            underground_name: String::new(),
            underground_edit_idx: None,
            zone_list: Vec::new(),
            zone_name: String::new(),
            zone_edit_idx: None,
            world_list: Vec::new(),
            world_name: String::new(),
            world_edit_idx: None,
            realm_list: Vec::new(),
            realm_name: String::new(),
            realm_edit_idx: None,
            show_race: false,
            show_class: false,
            show_profession: false,
            map_forge: MapForgeState::default(),
            race_list: vec![SimpleEntry { name: "Human".into() }, SimpleEntry { name: "Elf".into() }, SimpleEntry { name: "Dwarf".into() }, SimpleEntry { name: "Orc".into() }, SimpleEntry { name: "Goblin".into() }, SimpleEntry { name: "Dragonkin".into() }, SimpleEntry { name: "Undead".into() }, SimpleEntry { name: "Beast".into() }, SimpleEntry { name: "Demon".into() }, SimpleEntry { name: "Angel".into() }, SimpleEntry { name: "Construct".into() }],
            race_name: String::new(),
            race_edit_idx: None,
            class_list: vec![SimpleEntry { name: "Warrior".into() }, SimpleEntry { name: "Mage".into() }, SimpleEntry { name: "Thief".into() }, SimpleEntry { name: "Cleric".into() }, SimpleEntry { name: "Ranger".into() }, SimpleEntry { name: "Paladin".into() }, SimpleEntry { name: "Monk".into() }, SimpleEntry { name: "Bard".into() }, SimpleEntry { name: "Necromancer".into() }, SimpleEntry { name: "Summoner".into() }, SimpleEntry { name: "Berserker".into() }, SimpleEntry { name: "Knight".into() }, SimpleEntry { name: "Samurai".into() }, SimpleEntry { name: "Ninja".into() }, SimpleEntry { name: "Alchemist".into() }],
            class_name: String::new(),
            class_edit_idx: None,
            profession_list: vec![SimpleEntry { name: "Blacksmith".into() }, SimpleEntry { name: "Merchant".into() }, SimpleEntry { name: "Healer".into() }, SimpleEntry { name: "Hunter".into() }, SimpleEntry { name: "Farmer".into() }, SimpleEntry { name: "Fisher".into() }, SimpleEntry { name: "Miner".into() }, SimpleEntry { name: "Scholar".into() }, SimpleEntry { name: "Guard".into() }],
            profession_name: String::new(),
            profession_edit_idx: None,
            // New RPG/JRPG/MMORPG archetype data
            all_races: vec!["Human".into(), "Elf".into(), "Dwarf".into(), "Orc".into(), "Goblin".into(), "Dragonkin".into(), "Undead".into(), "Beast".into(), "Demon".into(), "Angel".into(), "Construct".into()],
            all_classes: vec!["Warrior".into(), "Mage".into(), "Thief".into(), "Cleric".into(), "Ranger".into(), "Paladin".into(), "Monk".into(), "Bard".into(), "Necromancer".into(), "Summoner".into(), "Berserker".into(), "Knight".into(), "Samurai".into(), "Ninja".into(), "Alchemist".into()],
            all_types: vec!["Friendly".into(), "Enemy".into(), "Merchant".into(), "QuestGiver".into(), "Boss".into(), "Miniboss".into(), "Elite".into(), "Common".into()],
            all_subtypes: vec!["Fire".into(), "Ice".into(), "Lightning".into(), "Poison".into(), "Holy".into(), "Dark".into(), "Mechanical".into(), "Undead".into(), "Beast".into()],
            all_items: vec![
                ItemEntry { name: "Potion".into(), item_type: ItemType::Consumable, subtype: "Healing".into(), description: "Restores HP".into(), stats: vec![("HP".into(), 50)], rarity: "Common".into(), value: 10, usable_in_battle: true, usable_in_field: true },
                ItemEntry { name: "Hi-Potion".into(), item_type: ItemType::Consumable, subtype: "Healing".into(), description: "Restores more HP".into(), stats: vec![("HP".into(), 200)], rarity: "Uncommon".into(), value: 50, usable_in_battle: true, usable_in_field: true },
                ItemEntry { name: "Antidote".into(), item_type: ItemType::Consumable, subtype: "Cure".into(), description: "Cures poison".into(), stats: vec![], rarity: "Common".into(), value: 8, usable_in_battle: true, usable_in_field: true },
                ItemEntry { name: "Elixir".into(), item_type: ItemType::Consumable, subtype: "Restoration".into(), description: "Fully restores HP/MP".into(), stats: vec![("HP".into(), 9999), ("MP".into(), 9999)], rarity: "Rare".into(), value: 500, usable_in_battle: true, usable_in_field: true },
            ],
            all_weapons: vec![
                WeaponEntry { name: "Iron Sword".into(), weapon_type: WeaponType::Sword, description: "A basic iron sword.".into(), stats: vec![("ATK".into(), 10)], rarity: "Common".into(), value: 30, usable_by: vec!["Warrior".into(), "Knight".into()] },
                WeaponEntry { name: "Magic Staff".into(), weapon_type: WeaponType::Staff, description: "A staff for casting spells.".into(), stats: vec![("ATK".into(), 3), ("MAG".into(), 8)], rarity: "Uncommon".into(), value: 60, usable_by: vec!["Mage".into(), "Cleric".into()] },
                WeaponEntry { name: "Dagger".into(), weapon_type: WeaponType::Dagger, description: "A small, quick blade.".into(), stats: vec![("ATK".into(), 6), ("AGI".into(), 2)], rarity: "Common".into(), value: 18, usable_by: vec!["Thief".into(), "Ninja".into()] },
            ],
            all_armors: vec![
                ArmorEntry { name: "Leather Armor".into(), armor_type: ArmorType::Chest, description: "Basic leather armor.".into(), stats: vec![("DEF".into(), 5)], rarity: "Common".into(), value: 20, usable_by: vec!["Warrior".into(), "Thief".into()] },
                ArmorEntry { name: "Iron Shield".into(), armor_type: ArmorType::Shield, description: "A sturdy iron shield.".into(), stats: vec![("DEF".into(), 8)], rarity: "Uncommon".into(), value: 35, usable_by: vec!["Warrior".into(), "Knight".into()] },
                ArmorEntry { name: "Wizard Robe".into(), armor_type: ArmorType::Chest, description: "A robe for spellcasters.".into(), stats: vec![("DEF".into(), 2), ("MAG".into(), 5)], rarity: "Uncommon".into(), value: 40, usable_by: vec!["Mage".into(), "Cleric".into()] },
            ],
            all_loot_tables: vec![
                LootTableEntry { item: "Potion".into(), chance: 0.5, min_qty: 1, max_qty: 2 },
                LootTableEntry { item: "Iron Sword".into(), chance: 0.1, min_qty: 1, max_qty: 1 },
                LootTableEntry { item: "Leather Armor".into(), chance: 0.08, min_qty: 1, max_qty: 1 },
            ],
            all_npcs: Vec::new(),
            all_enemies: Vec::new(),
            // Items menu state
            item_name: String::new(),
            item_desc: String::new(),
            item_value: 0,
            item_edit_idx: None,
        }
    }

    /// Show the main feature tool GUI, including all toolbars and editors.
    ///
    /// This method draws all major toolbars, tool dialogs, and feature editors.
    /// Extend this method to add new tool sections, reorganize the UI, or add new feature dialogs.
    ///
    /// # Arguments
    /// * `ctx` - The egui context for rendering the UI.
    pub fn show(&mut self, ctx: &egui::Context) {
        // Left vertical toolbar with more tools and submenus
        egui::SidePanel::left("left_toolbar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Tools");
                if ui.button("Show MapForge").clicked() {
                    self.map_forge.toggle();
                }
                if ui.button("World Editor").clicked() {
                    println!("World Editor opened");
                }
                if ui.button("Engine Settings").clicked() {
                    println!("Engine Settings opened");
                }
                if ui.button("Debug Console").clicked() {
                    println!("Debug Console opened");
                }
                ui.separator();
                ui.collapsing("Quick Actions", |ui| {
                    if ui.button("Save Game").clicked() {
                        println!("Game saved");
                    }
                    if ui.button("Load Game").clicked() {
                        println!("Game loaded");
                    }
                    if ui.button("Run Test").clicked() {
                        println!("Test run");
                    }
                });
            });
        });

        // MapForge Black Screen Panel (overlay)
        if self.map_forge.visible {
            egui::CentralPanel::default().show(ctx, |ui| {
                self.map_forge.show_toolbar(ui);
                self.map_forge.show_canvas(ui);
            });
        }

        // Right vertical panel for inspector, debug, and systems
        egui::SidePanel::right("right_toolbar").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Inspector & Systems");
                if ui.button("Inspector").clicked() {
                    println!("Inspector opened");
                }
                if ui.button("Hierarchy").clicked() {
                    println!("Hierarchy opened");
                }
                if ui.button("Profiler").clicked() {
                    println!("Profiler opened");
                }
                if ui.button("Log Viewer").clicked() {
                    println!("Log Viewer opened");
                }
                ui.separator();
                ui.collapsing("Systems", |ui| {
                    if ui.button("Combat System").clicked() {
                        println!("Combat System opened");
                    }
                    if ui.button("Inventory System").clicked() {
                        println!("Inventory System opened");
                    }
                    if ui.button("Dialogue System").clicked() {
                        println!("Dialogue System opened");
                    }
                });
            });
        });

        // Top horizontal toolbar
        egui::TopBottomPanel::top("top_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Render feature menus recursively
                let menus = build_feature_menu();
                for menu in &menus {
                    self.render_menu_recursive(ui, menu);
                }
                ui.separator();
                // Existing pixel asset tools
                if ui.button("Generate 8-bit Sprite Sheet").clicked() {
                    generate_8bit_sprite_sheet();
                }
                if ui.button("Export Logo Sprite Atlas").clicked() {
                    export_logo_sprite_atlas();
                }
                if ui.button("Animate Logo Glow").clicked() {
                    animate_logo_glow();
                }
                if ui.button("Create Retro Bitmap Font").clicked() {
                    create_retro_bitmap_font();
                }
                if ui.button("Build Pixel UI Assets").clicked() {
                    build_pixel_ui_assets();
                }
            });
        });

        // Modern API GUI layout: Sidebar + Split View
        egui::SidePanel::left("api_sidebar").default_width(180.0).show(ctx, |ui| {
            ui.heading("API Tools");
            ui.separator();
            if ui.button("🔌 Test Connection").clicked() {
                println!("API connection test triggered");
            }
            if ui.button("🔄 Reload Schema").clicked() {
                println!("API schema reload triggered");
            }
            if ui.button("📄 Open Docs").clicked() {
                println!("Open API docs triggered");
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("API Results / Details");
            ui.separator();
            ui.label("Results, logs, or API responses will appear here.");
            // TODO: Add dynamic content based on API actions
        });

        // API Tools Section
        ui.separator();
        ui.heading("API Tools");
        ui.horizontal(|ui| {
            if ui.button("Test API Connection").clicked() {
                println!("API connection test triggered");
            }
            if ui.button("Reload API Schema").clicked() {
                println!("API schema reload triggered");
            }
            if ui.button("Open API Docs").clicked() {
                println!("Open API docs triggered");
            }
        });

        // Main MMORPG Tools Section
        egui::Window::new("MMORPG Tools").show(ctx, |ui| {
            if ui.button("Open Blueprint Editor").clicked() { self.show_blueprint_editor = true; }
                                if self.show_blueprint_editor {
                                    self.blueprint_editor.show(ctx);
                                }
                        if ui.button("Open Race Editor").clicked() { self.show_race = true; }
                        if ui.button("Open Class Editor").clicked() { self.show_class = true; }
                        if ui.button("Open Profession Editor").clicked() { self.show_profession = true; }
                    if self.show_race {
                        egui::Window::new("Race Editor").open(&mut self.show_race).show(ctx, |ui| {
                            ui.heading("Race List");
                            let mut remove_idx = None;
                            for (i, entry) in self.race_list.iter().enumerate() {
                                ui.horizontal(|h| {
                                    h.label(&entry.name);
                                    if h.button("Edit").clicked() {
                                        self.race_name = entry.name.clone();
                                        self.race_edit_idx = Some(i);
                                    }
                                    if h.button("Remove").clicked() {
                                        remove_idx = Some(i);
                                    }
                                });
                            }
                            if let Some(idx) = remove_idx {
                                self.race_list.remove(idx);
                            }
                            ui.separator();
                            if let Some(edit_idx) = self.race_edit_idx {
                                ui.label("Edit Race:");
                                ui.text_edit_singleline(&mut self.race_name);
                                if ui.button("Save").clicked() {
                                    if !self.race_name.trim().is_empty() {
                                        self.race_list[edit_idx] = SimpleEntry { name: self.race_name.clone() };
                                        self.race_edit_idx = None;
                                        self.race_name.clear();
                                    }
                                }
                                if ui.button("Cancel").clicked() {
                                    self.race_edit_idx = None;
                                    self.race_name.clear();
                                }
                            } else {
                                ui.label("Add New Race:");
                                ui.text_edit_singleline(&mut self.race_name);
                                if ui.button("Add").clicked() {
                                    if !self.race_name.trim().is_empty() {
                                        self.race_list.push(SimpleEntry { name: self.race_name.clone() });
                                        self.race_name.clear();
                                    }
                                }
                            }
                        });
                    }
                    if self.show_class {
                        egui::Window::new("Class Editor").open(&mut self.show_class).show(ctx, |ui| {
                            ui.heading("Class List");
                            let mut remove_idx = None;
                            for (i, entry) in self.class_list.iter().enumerate() {
                                ui.horizontal(|h| {
                                    h.label(&entry.name);
                                    if h.button("Edit").clicked() {
                                        self.class_name = entry.name.clone();
                                        self.class_edit_idx = Some(i);
                                    }
                                    if h.button("Remove").clicked() {
                                        remove_idx = Some(i);
                                    }
                                });
                            }
                            if let Some(idx) = remove_idx {
                                self.class_list.remove(idx);
                            }
                            ui.separator();
                            if let Some(edit_idx) = self.class_edit_idx {
                                ui.label("Edit Class:");
                                ui.text_edit_singleline(&mut self.class_name);
                                if ui.button("Save").clicked() {
                                    if !self.class_name.trim().is_empty() {
                                        self.class_list[edit_idx] = SimpleEntry { name: self.class_name.clone() };
                                        self.class_edit_idx = None;
                                        self.class_name.clear();
                                    }
                                }
                                if ui.button("Cancel").clicked() {
                                    self.class_edit_idx = None;
                                    self.class_name.clear();
                                }
                            } else {
                                ui.label("Add New Class:");
                                ui.text_edit_singleline(&mut self.class_name);
                                if ui.button("Add").clicked() {
                                    if !self.class_name.trim().is_empty() {
                                        self.class_list.push(SimpleEntry { name: self.class_name.clone() });
                                        self.class_name.clear();
                                    }
                                }
                            }
                        });
                    }
                    if self.show_profession {
                        egui::Window::new("Profession Editor").open(&mut self.show_profession).show(ctx, |ui| {
                            ui.heading("Profession List");
                            let mut remove_idx = None;
                            for (i, entry) in self.profession_list.iter().enumerate() {
                                ui.horizontal(|h| {
                                    h.label(&entry.name);
                                    if h.button("Edit").clicked() {
                                        self.profession_name = entry.name.clone();
                                        self.profession_edit_idx = Some(i);
                                    }
                                    if h.button("Remove").clicked() {
                                        remove_idx = Some(i);
                                    }
                                });
                            }
                            if let Some(idx) = remove_idx {
                                self.profession_list.remove(idx);
                            }
                            ui.separator();
                            if let Some(edit_idx) = self.profession_edit_idx {
                                ui.label("Edit Profession:");
                                ui.text_edit_singleline(&mut self.profession_name);
                                if ui.button("Save").clicked() {
                                    if !self.profession_name.trim().is_empty() {
                                        self.profession_list[edit_idx] = SimpleEntry { name: self.profession_name.clone() };
                                        self.profession_edit_idx = None;
                                        self.profession_name.clear();
                                    }
                                }
                                if ui.button("Cancel").clicked() {
                                    self.profession_edit_idx = None;
                                    self.profession_name.clear();
                                }
                            } else {
                                ui.label("Add New Profession:");
                                ui.text_edit_singleline(&mut self.profession_name);
                                if ui.button("Add").clicked() {
                                    if !self.profession_name.trim().is_empty() {
                                        self.profession_list.push(SimpleEntry { name: self.profession_name.clone() });
                                        self.profession_name.clear();
                                    }
                                }
                            }
                        });
                    }
            ui.heading("MMORPG Feature Tools");
            if ui.button("Open Loot Editor").clicked() { self.show_loot = true; }
            if ui.button("Open Skill Tree Editor").clicked() { self.show_skill_tree = true; }
            if ui.button("Open Enemy/Boss Editor").clicked() { self.show_enemy_boss = true; }
            if ui.button("Open Spell Editor").clicked() { self.show_spell = true; }
            if ui.button("Open Level System Editor").clicked() { self.show_level = true; }
            if ui.button("Open Equipment Editor").clicked() { self.show_equipment = true; }
            if ui.button("Open Inventory Editor").clicked() { self.show_inventory = true; }
            if ui.button("Open Stat/Attribute Editor").clicked() { self.show_stat = true; }
            if ui.button("Open Item Tier Editor").clicked() { self.show_item_tier = true; }
            if ui.button("Open Biome Editor").clicked() { self.show_biome = true; }
            if ui.button("Open Encounter Editor").clicked() { self.show_encounter = true; }
            if ui.button("Open Group Finder Editor").clicked() { self.show_group_finder = true; }
            if ui.button("Open World Boss Editor").clicked() { self.show_world_boss = true; }
            if ui.button("Open Event Editor").clicked() { self.show_event = true; }
            if ui.button("Open World Map/Planet Editor").clicked() { self.show_world_map = true; }
            if ui.button("Open Story Editor").clicked() { self.show_story = true; }
            if ui.button("Open Quest Editor").clicked() { self.show_quest = true; }
            if ui.button("Open Side Quest Editor").clicked() { self.show_side_quest = true; }
            if ui.button("Open Act Editor").clicked() { self.show_act = true; }
            if ui.button("Open Chapter Editor").clicked() { self.show_chapter = true; }
            if ui.button("Open Rewards Editor").clicked() { self.show_rewards = true; }
            if ui.button("Open City Editor").clicked() { self.show_city = true; }
            if ui.button("Open Town Editor").clicked() { self.show_town = true; }
            if ui.button("Open Kingdom Editor").clicked() { self.show_kingdom = true; }
            if ui.button("Open Underground Editor").clicked() { self.show_underground = true; }
            if ui.button("Open Zone Editor").clicked() { self.show_zone = true; }
            if ui.button("Open World Editor").clicked() { self.show_world = true; }
            if ui.button("Open Realm Editor").clicked() { self.show_realm = true; }
        });
                if self.show_city {
                    egui::Window::new("City Editor").open(&mut self.show_city).show(ctx, |ui| {
                        ui.heading("City List");
                        let mut remove_idx = None;
                        for (i, entry) in self.city_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.city_name = entry.name.clone();
                                    self.city_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.city_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.city_edit_idx {
                            ui.label("Edit City:");
                            ui.text_edit_singleline(&mut self.city_name);
                            if ui.button("Save").clicked() {
                                if !self.city_name.trim().is_empty() {
                                    self.city_list[edit_idx] = SimpleEntry { name: self.city_name.clone() };
                                    self.city_edit_idx = None;
                                    self.city_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.city_edit_idx = None;
                                self.city_name.clear();
                            }
                        } else {
                            ui.label("Add New City:");
                            ui.text_edit_singleline(&mut self.city_name);
                            if ui.button("Add").clicked() {
                                if !self.city_name.trim().is_empty() {
                                    self.city_list.push(SimpleEntry { name: self.city_name.clone() });
                                    self.city_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_town {
                    egui::Window::new("Town Editor").open(&mut self.show_town).show(ctx, |ui| {
                        ui.heading("Town List");
                        let mut remove_idx = None;
                        for (i, entry) in self.town_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.town_name = entry.name.clone();
                                    self.town_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.town_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.town_edit_idx {
                            ui.label("Edit Town:");
                            ui.text_edit_singleline(&mut self.town_name);
                            if ui.button("Save").clicked() {
                                if !self.town_name.trim().is_empty() {
                                    self.town_list[edit_idx] = SimpleEntry { name: self.town_name.clone() };
                                    self.town_edit_idx = None;
                                    self.town_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.town_edit_idx = None;
                                self.town_name.clear();
                            }
                        } else {
                            ui.label("Add New Town:");
                            ui.text_edit_singleline(&mut self.town_name);
                            if ui.button("Add").clicked() {
                                if !self.town_name.trim().is_empty() {
                                    self.town_list.push(SimpleEntry { name: self.town_name.clone() });
                                    self.town_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_kingdom {
                    egui::Window::new("Kingdom Editor").open(&mut self.show_kingdom).show(ctx, |ui| {
                        ui.heading("Kingdom List");
                        let mut remove_idx = None;
                        for (i, entry) in self.kingdom_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.kingdom_name = entry.name.clone();
                                    self.kingdom_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.kingdom_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.kingdom_edit_idx {
                            ui.label("Edit Kingdom:");
                            ui.text_edit_singleline(&mut self.kingdom_name);
                            if ui.button("Save").clicked() {
                                if !self.kingdom_name.trim().is_empty() {
                                    self.kingdom_list[edit_idx] = SimpleEntry { name: self.kingdom_name.clone() };
                                    self.kingdom_edit_idx = None;
                                    self.kingdom_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.kingdom_edit_idx = None;
                                self.kingdom_name.clear();
                            }
                        } else {
                            ui.label("Add New Kingdom:");
                            ui.text_edit_singleline(&mut self.kingdom_name);
                            if ui.button("Add").clicked() {
                                if !self.kingdom_name.trim().is_empty() {
                                    self.kingdom_list.push(SimpleEntry { name: self.kingdom_name.clone() });
                                    self.kingdom_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_underground {
                    egui::Window::new("Underground Editor").open(&mut self.show_underground).show(ctx, |ui| {
                        ui.heading("Underground List");
                        let mut remove_idx = None;
                        for (i, entry) in self.underground_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.underground_name = entry.name.clone();
                                    self.underground_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.underground_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.underground_edit_idx {
                            ui.label("Edit Underground:");
                            ui.text_edit_singleline(&mut self.underground_name);
                            if ui.button("Save").clicked() {
                                if !self.underground_name.trim().is_empty() {
                                    self.underground_list[edit_idx] = SimpleEntry { name: self.underground_name.clone() };
                                    self.underground_edit_idx = None;
                                    self.underground_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.underground_edit_idx = None;
                                self.underground_name.clear();
                            }
                        } else {
                            ui.label("Add New Underground:");
                            ui.text_edit_singleline(&mut self.underground_name);
                            if ui.button("Add").clicked() {
                                if !self.underground_name.trim().is_empty() {
                                    self.underground_list.push(SimpleEntry { name: self.underground_name.clone() });
                                    self.underground_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_zone {
                    egui::Window::new("Zone Editor").open(&mut self.show_zone).show(ctx, |ui| {
                        ui.heading("Zone List");
                        let mut remove_idx = None;
                        for (i, entry) in self.zone_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.zone_name = entry.name.clone();
                                    self.zone_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.zone_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.zone_edit_idx {
                            ui.label("Edit Zone:");
                            ui.text_edit_singleline(&mut self.zone_name);
                            if ui.button("Save").clicked() {
                                if !self.zone_name.trim().is_empty() {
                                    self.zone_list[edit_idx] = SimpleEntry { name: self.zone_name.clone() };
                                    self.zone_edit_idx = None;
                                    self.zone_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.zone_edit_idx = None;
                                self.zone_name.clear();
                            }
                        } else {
                            ui.label("Add New Zone:");
                            ui.text_edit_singleline(&mut self.zone_name);
                            if ui.button("Add").clicked() {
                                if !self.zone_name.trim().is_empty() {
                                    self.zone_list.push(SimpleEntry { name: self.zone_name.clone() });
                                    self.zone_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_world {
                    egui::Window::new("World Editor").open(&mut self.show_world).show(ctx, |ui| {
                        ui.heading("World List");
                        let mut remove_idx = None;
                        for (i, entry) in self.world_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.world_name = entry.name.clone();
                                    self.world_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.world_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.world_edit_idx {
                            ui.label("Edit World:");
                            ui.text_edit_singleline(&mut self.world_name);
                            if ui.button("Save").clicked() {
                                if !self.world_name.trim().is_empty() {
                                    self.world_list[edit_idx] = SimpleEntry { name: self.world_name.clone() };
                                    self.world_edit_idx = None;
                                    self.world_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.world_edit_idx = None;
                                self.world_name.clear();
                            }
                        } else {
                            ui.label("Add New World:");
                            ui.text_edit_singleline(&mut self.world_name);
                            if ui.button("Add").clicked() {
                                if !self.world_name.trim().is_empty() {
                                    self.world_list.push(SimpleEntry { name: self.world_name.clone() });
                                    self.world_name.clear();
                                }
                            }
                        }
                    });
                }
                if self.show_realm {
                    egui::Window::new("Realm Editor").open(&mut self.show_realm).show(ctx, |ui| {
                        ui.heading("Realm List");
                        let mut remove_idx = None;
                        for (i, entry) in self.realm_list.iter().enumerate() {
                            ui.horizontal(|h| {
                                h.label(&entry.name);
                                if h.button("Edit").clicked() {
                                    self.realm_name = entry.name.clone();
                                    self.realm_edit_idx = Some(i);
                                }
                                if h.button("Remove").clicked() {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(idx) = remove_idx {
                            self.realm_list.remove(idx);
                        }
                        ui.separator();
                        if let Some(edit_idx) = self.realm_edit_idx {
                            ui.label("Edit Realm:");
                            ui.text_edit_singleline(&mut self.realm_name);
                            if ui.button("Save").clicked() {
                                if !self.realm_name.trim().is_empty() {
                                    self.realm_list[edit_idx] = SimpleEntry { name: self.realm_name.clone() };
                                    self.realm_edit_idx = None;
                                    self.realm_name.clear();
                                }
                            }
                            if ui.button("Cancel").clicked() {
                                self.realm_edit_idx = None;
                                self.realm_name.clear();
                            }
                        } else {
                            ui.label("Add New Realm:");
                            ui.text_edit_singleline(&mut self.realm_name);
                            if ui.button("Add").clicked() {
                                if !self.realm_name.trim().is_empty() {
                                    self.realm_list.push(SimpleEntry { name: self.realm_name.clone() });
                                    self.realm_name.clear();
                                }
                            }
                        }
                    });
                }
        if self.show_story {
            egui::Window::new("Story Editor").open(&mut self.show_story).show(ctx, |ui| {
                ui.heading("Story List");
                let mut remove_idx = None;
                for (i, story) in self.story_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&story.title);
                        if h.button("Edit").clicked() {
                            self.story_title = story.title.clone();
                            self.story_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.story_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.story_edit_idx {
                    ui.label("Edit Story:");
                    ui.text_edit_singleline(&mut self.story_title);
                    if ui.button("Save").clicked() {
                        if !self.story_title.trim().is_empty() {
                            self.story_list[edit_idx] = StoryEntry { title: self.story_title.clone() };
                            self.story_edit_idx = None;
                            self.story_title.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.story_edit_idx = None;
                        self.story_title.clear();
                    }
                } else {
                    ui.label("Add New Story:");
                    ui.text_edit_singleline(&mut self.story_title);
                    if ui.button("Add").clicked() {
                        if !self.story_title.trim().is_empty() {
                            self.story_list.push(StoryEntry { title: self.story_title.clone() });
                            self.story_title.clear();
                        }
                    }
                }
            });
        }
        if self.show_world_map {
            egui::Window::new("World Map / Planet Editor").open(&mut self.show_world_map).show(ctx, |ui| {
                ui.heading("World Map / Planet Editor");
                ui.horizontal(|h| {
                    if h.button("2D View").clicked() {
                        // TODO: Switch to 2D view in WorldMapEditor
                    }
                    if h.button("3D View").clicked() {
                        // TODO: Switch to 3D view in WorldMapEditor
                    }
                });
                ui.separator();
                ui.label("Region Management:");
                ui.horizontal(|h| {
                    if h.button("Load Region").clicked() {
                        // TODO: Call WorldPartition::load_region
                    }
                    if h.button("Unload Region").clicked() {
                        // TODO: Call WorldPartition::unload_region
                    }
                });
                ui.separator();
                ui.label("Naming Tools:");
                ui.horizontal(|h| {
                    if h.button("Name Continent").clicked() {
                        // TODO: Open naming tool for continents
                    }
                    if h.button("Name Zone").clicked() {
                        // TODO: Open naming tool for zones
                    }
                    if h.button("Name Subzone").clicked() {
                        // TODO: Open naming tool for subzones
                    }
                });
                // TODO: Add world map rendering and editing UI here
            });
        }
        if self.show_quest {
            egui::Window::new("Quest Editor").open(&mut self.show_quest).show(ctx, |ui| {
                ui.heading("Quest List");
                let mut remove_idx = None;
                for (i, quest) in self.quest_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&quest.title);
                        if h.button("Edit").clicked() {
                            self.quest_title = quest.title.clone();
                            self.quest_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.quest_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.quest_edit_idx {
                    ui.label("Edit Quest:");
                    ui.text_edit_singleline(&mut self.quest_title);
                    if ui.button("Save").clicked() {
                        if !self.quest_title.trim().is_empty() {
                            self.quest_list[edit_idx] = QuestEntry { title: self.quest_title.clone() };
                            self.quest_edit_idx = None;
                            self.quest_title.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.quest_edit_idx = None;
                        self.quest_title.clear();
                    }
                } else {
                    ui.label("Add New Quest:");
                    ui.text_edit_singleline(&mut self.quest_title);
                    if ui.button("Add").clicked() {
                        if !self.quest_title.trim().is_empty() {
                            self.quest_list.push(QuestEntry { title: self.quest_title.clone() });
                            self.quest_title.clear();
                        }
                    }
                }
            });
        }
        if self.show_side_quest {
            egui::Window::new("Side Quest Editor").open(&mut self.show_side_quest).show(ctx, |ui| {
                ui.heading("Side Quest List");
                let mut remove_idx = None;
                for (i, quest) in self.side_quest_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&quest.title);
                        if h.button("Edit").clicked() {
                            self.side_quest_title = quest.title.clone();
                            self.side_quest_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.side_quest_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.side_quest_edit_idx {
                    ui.label("Edit Side Quest:");
                    ui.text_edit_singleline(&mut self.side_quest_title);
                    if ui.button("Save").clicked() {
                        if !self.side_quest_title.trim().is_empty() {
                            self.side_quest_list[edit_idx] = QuestEntry { title: self.side_quest_title.clone() };
                            self.side_quest_edit_idx = None;
                            self.side_quest_title.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.side_quest_edit_idx = None;
                        self.side_quest_title.clear();
                    }
                } else {
                    ui.label("Add New Side Quest:");
                    ui.text_edit_singleline(&mut self.side_quest_title);
                    if ui.button("Add").clicked() {
                        if !self.side_quest_title.trim().is_empty() {
                            self.side_quest_list.push(QuestEntry { title: self.side_quest_title.clone() });
                            self.side_quest_title.clear();
                        }
                    }
                }
            });
        }
        if self.show_act {
            egui::Window::new("Act Editor").open(&mut self.show_act).show(ctx, |ui| {
                ui.heading("Act List");
                let mut remove_idx = None;
                for (i, act) in self.act_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&act.title);
                        if h.button("Edit").clicked() {
                            self.act_title = act.title.clone();
                            self.act_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.act_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.act_edit_idx {
                    ui.label("Edit Act:");
                    ui.text_edit_singleline(&mut self.act_title);
                    if ui.button("Save").clicked() {
                        if !self.act_title.trim().is_empty() {
                            self.act_list[edit_idx] = ActEntry { title: self.act_title.clone() };
                            self.act_edit_idx = None;
                            self.act_title.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.act_edit_idx = None;
                        self.act_title.clear();
                    }
                } else {
                    ui.label("Add New Act:");
                    ui.text_edit_singleline(&mut self.act_title);
                    if ui.button("Add").clicked() {
                        if !self.act_title.trim().is_empty() {
                            self.act_list.push(ActEntry { title: self.act_title.clone() });
                            self.act_title.clear();
                        }
                    }
                }
            });
        }
        if self.show_chapter {
            egui::Window::new("Chapter Editor").open(&mut self.show_chapter).show(ctx, |ui| {
                ui.heading("Chapter List");
                let mut remove_idx = None;
                for (i, chapter) in self.chapter_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&chapter.title);
                        if h.button("Edit").clicked() {
                            self.chapter_title = chapter.title.clone();
                            self.chapter_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.chapter_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.chapter_edit_idx {
                    ui.label("Edit Chapter:");
                    ui.text_edit_singleline(&mut self.chapter_title);
                    if ui.button("Save").clicked() {
                        if !self.chapter_title.trim().is_empty() {
                            self.chapter_list[edit_idx] = ChapterEntry { title: self.chapter_title.clone() };
                            self.chapter_edit_idx = None;
                            self.chapter_title.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.chapter_edit_idx = None;
                        self.chapter_title.clear();
                    }
                } else {
                    ui.label("Add New Chapter:");
                    ui.text_edit_singleline(&mut self.chapter_title);
                    if ui.button("Add").clicked() {
                        if !self.chapter_title.trim().is_empty() {
                            self.chapter_list.push(ChapterEntry { title: self.chapter_title.clone() });
                            self.chapter_title.clear();
                        }
                    }
                }
            });
        }
        if self.show_rewards {
            egui::Window::new("Rewards Editor").open(&mut self.show_rewards).show(ctx, |ui| {
                ui.heading("Rewards List");
                let mut remove_idx = None;
                for (i, reward) in self.reward_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(&reward.desc);
                        if h.button("Edit").clicked() {
                            self.reward_desc = reward.desc.clone();
                            self.reward_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.reward_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.reward_edit_idx {
                    ui.label("Edit Reward:");
                    ui.text_edit_singleline(&mut self.reward_desc);
                    if ui.button("Save").clicked() {
                        if !self.reward_desc.trim().is_empty() {
                            self.reward_list[edit_idx] = RewardEntry { desc: self.reward_desc.clone() };
                            self.reward_edit_idx = None;
                            self.reward_desc.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.reward_edit_idx = None;
                        self.reward_desc.clear();
                    }
                } else {
                    ui.label("Add New Reward:");
                    ui.text_edit_singleline(&mut self.reward_desc);
                    if ui.button("Add").clicked() {
                        if !self.reward_desc.trim().is_empty() {
                            self.reward_list.push(RewardEntry { desc: self.reward_desc.clone() });
                            self.reward_desc.clear();
                        }
                    }
                }
            });
        }

        if self.show_loot {
            egui::Window::new("Loot Editor").open(&mut self.show_loot).show(ctx, |ui| {
                ui.heading("Loot Table");
                // List loot entries
                let mut remove_idx = None;
                for (i, entry) in self.loot_table.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(format!("{} (Chance: {:.2}%)", entry.name, entry.chance));
                        if h.button("Edit").clicked() {
                            self.loot_name = entry.name.clone();
                            self.loot_chance = entry.chance;
                            self.loot_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.loot_table.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.loot_edit_idx {
                    ui.label("Edit Loot Entry:");
                    ui.text_edit_singleline(&mut self.loot_name);
                    ui.add(egui::Slider::new(&mut self.loot_chance, 0.0..=100.0).text("Chance %"));
                    if ui.button("Save").clicked() {
                        if !self.loot_name.trim().is_empty() {
                            self.loot_table[edit_idx] = LootEntry { name: self.loot_name.clone(), chance: self.loot_chance };
                            self.loot_edit_idx = None;
                            self.loot_name.clear();
                            self.loot_chance = 1.0;
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.loot_edit_idx = None;
                        self.loot_name.clear();
                        self.loot_chance = 1.0;
                    }
                } else {
                    ui.label("Add New Loot Entry:");
                    ui.text_edit_singleline(&mut self.loot_name);
                    ui.add(egui::Slider::new(&mut self.loot_chance, 0.0..=100.0).text("Chance %"));
                    if ui.button("Add").clicked() {
                        if !self.loot_name.trim().is_empty() {
                            self.loot_table.push(LootEntry { name: self.loot_name.clone(), chance: self.loot_chance });
                            self.loot_name.clear();
                            self.loot_chance = 1.0;
                        }
                    }
                }
            });
        }
        if self.show_skill_tree {
            egui::Window::new("Skill Tree Editor").open(&mut self.show_skill_tree).show(ctx, |ui| {
                ui.heading("Skill Tree");
                // List skills
                let mut remove_idx = None;
                for (i, skill) in self.skill_tree.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(format!("{}: {}", skill.name, skill.desc));
                        if h.button("Edit").clicked() {
                            self.skill_name = skill.name.clone();
                            self.skill_desc = skill.desc.clone();
                            self.skill_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.skill_tree.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.skill_edit_idx {
                    ui.label("Edit Skill:");
                    ui.text_edit_singleline(&mut self.skill_name);
                    ui.text_edit_singleline(&mut self.skill_desc);
                    if ui.button("Save").clicked() {
                        if !self.skill_name.trim().is_empty() {
                            self.skill_tree[edit_idx] = SkillEntry { name: self.skill_name.clone(), desc: self.skill_desc.clone() };
                            self.skill_edit_idx = None;
                            self.skill_name.clear();
                            self.skill_desc.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.skill_edit_idx = None;
                        self.skill_name.clear();
                        self.skill_desc.clear();
                    }
                } else {
                    ui.label("Add New Skill:");
                    ui.text_edit_singleline(&mut self.skill_name);
                    ui.text_edit_singleline(&mut self.skill_desc);
                    if ui.button("Add").clicked() {
                        if !self.skill_name.trim().is_empty() {
                            self.skill_tree.push(SkillEntry { name: self.skill_name.clone(), desc: self.skill_desc.clone() });
                            self.skill_name.clear();
                            self.skill_desc.clear();
                        }
                    }
                }
            });
        }
        if self.show_enemy_boss {
            egui::Window::new("Enemy/Boss Editor").open(&mut self.show_enemy_boss).show(ctx, |ui| {
                ui.heading("Enemy/Boss List");
                let mut remove_idx = None;
                for (i, enemy) in self.enemy_list.iter().enumerate() {
                    ui.horizontal(|h| {
                        h.label(format!("{} [{}]", enemy.name, enemy.kind));
                        if h.button("Edit").clicked() {
                            self.enemy_name = enemy.name.clone();
                            self.enemy_type = enemy.kind.clone();
                            self.enemy_edit_idx = Some(i);
                        }
                        if h.button("Remove").clicked() {
                            remove_idx = Some(i);
                        }
                    });
                }
                if let Some(idx) = remove_idx {
                    self.enemy_list.remove(idx);
                }
                ui.separator();
                if let Some(edit_idx) = self.enemy_edit_idx {
                    ui.label("Edit Enemy/Boss:");
                    ui.text_edit_singleline(&mut self.enemy_name);
                    ui.text_edit_singleline(&mut self.enemy_type);
                    if ui.button("Save").clicked() {
                        if !self.enemy_name.trim().is_empty() {
                            self.enemy_list[edit_idx] = EnemyEntry { name: self.enemy_name.clone(), kind: self.enemy_type.clone() };
                            self.enemy_edit_idx = None;
                            self.enemy_name.clear();
                            self.enemy_type.clear();
                        }
                    }
                    if ui.button("Cancel").clicked() {
                        self.enemy_edit_idx = None;
                        self.enemy_name.clear();
                        self.enemy_type.clear();
                    }
                } else {
                    ui.label("Add New Enemy/Boss:");
                    ui.text_edit_singleline(&mut self.enemy_name);
                    ui.text_edit_singleline(&mut self.enemy_type);
                    if ui.button("Add").clicked() {
                        if !self.enemy_name.trim().is_empty() {
                            self.enemy_list.push(EnemyEntry { name: self.enemy_name.clone(), kind: self.enemy_type.clone() });
                            self.enemy_name.clear();
                            self.enemy_type.clear();
                        }
                    }
                }
            });
        }
        if self.show_spell {
            egui::Window::new("Spell Editor").open(&mut self.show_spell).show(ctx, |ui| {
                ui.label("Spell/Effect Editor (scaffold)");
            });
        }
        if self.show_level {
            egui::Window::new("Level System Editor").open(&mut self.show_level).show(ctx, |ui| {
                ui.label("Level/XP System Editor (scaffold)");
            });
        }
        if self.show_equipment {
            egui::Window::new("Equipment Editor").open(&mut self.show_equipment).show(ctx, |ui| {
                ui.label("Equipment System Editor (scaffold)");
            });
        }
        if self.show_inventory {
            egui::Window::new("Inventory Editor").open(&mut self.show_inventory).show(ctx, |ui| {
                ui.label("Inventory System Editor (scaffold)");
            });
        }
        if self.show_stat {
            egui::Window::new("Stat/Attribute Editor").open(&mut self.show_stat).show(ctx, |ui| {
                ui.label("Stat/Attribute System Editor (scaffold)");
            });
        }
        if self.show_item_tier {
            egui::Window::new("Item Tier Editor").open(&mut self.show_item_tier).show(ctx, |ui| {
                ui.label("Item Tier/Rarity Editor (scaffold)");
            });
        }
        if self.show_biome {
            egui::Window::new("Biome Editor").open(&mut self.show_biome).show(ctx, |ui| {
                ui.label("Biome/Environment System Editor (scaffold)");
            });
        }
        if self.show_encounter {
            egui::Window::new("Encounter Editor").open(&mut self.show_encounter).show(ctx, |ui| {
                ui.label("Encounter/Trial/Dungeon/Raid/Tower Editor (scaffold)");
            });
        }
        if self.show_group_finder {
            egui::Window::new("Group Finder Editor").open(&mut self.show_group_finder).show(ctx, |ui| {
                ui.label("Group Finder System Editor (scaffold)");
            });
        }
        if self.show_world_boss {
            egui::Window::new("World Boss Editor").open(&mut self.show_world_boss).show(ctx, |ui| {
                ui.label("World Boss/Nemesis System Editor (scaffold)");
            });
        }
        if self.show_event {
            egui::Window::new("Event Editor").open(&mut self.show_event).show(ctx, |ui| {
                ui.label("Event/Special Event System Editor (scaffold)");
            });
        }
    }
}
