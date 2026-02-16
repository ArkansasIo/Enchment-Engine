use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum CharacterClass {
    Warrior,
    Ranger,
    Mage,
    Cleric,
    Rogue,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Fire,
    Frost,
    Arcane,
    Holy,
    Shadow,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StatBlock {
    pub hp: i32,
    pub mp: i32,
    pub attack: i32,
    pub defense: i32,
    pub spell_power: i32,
    pub crit_chance: f32,
    pub haste: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
    pub damage_type: DamageType,
    pub base_power: i32,
    pub mana_cost: i32,
    pub cooldown_ms: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CharacterProfile {
    pub id: String,
    pub name: String,
    pub class: CharacterClass,
    pub level: u32,
    pub xp: u64,
    pub stats: StatBlock,
    pub unlocked_skills: BTreeSet<String>,
    pub equipped_item_level: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CombatEvent {
    pub attacker_id: String,
    pub defender_id: String,
    pub skill_id: Option<String>,
    pub raw_damage: i32,
    pub mitigated_damage: i32,
    pub did_crit: bool,
    pub defender_hp_after: i32,
    pub defeated: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LootTableEntry {
    pub item_id: String,
    pub rarity: ItemRarity,
    pub min_qty: u32,
    pub max_qty: u32,
    pub weight: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LootDrop {
    pub item_id: String,
    pub rarity: ItemRarity,
    pub quantity: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestObjective {
    pub objective_id: String,
    pub description: String,
    pub required: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestTemplate {
    pub quest_id: String,
    pub title: String,
    pub min_level: u32,
    pub objectives: Vec<QuestObjective>,
    pub reward_xp: u64,
    pub reward_gold: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuestProgress {
    pub quest_id: String,
    pub objective_progress: BTreeMap<String, u32>,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Guild {
    pub guild_id: String,
    pub name: String,
    pub level: u32,
    pub members: Vec<String>,
    pub message_of_the_day: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Party {
    pub party_id: String,
    pub leader: String,
    pub members: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldEventState {
    pub event_id: String,
    pub title: String,
    pub starts_at_ms: u64,
    pub ends_at_ms: u64,
    pub active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub world_name: String,
    pub server_tick_ms: u64,
    pub world_time_ms: u64,
    pub max_players_per_shard: u32,
    pub parties: Vec<Party>,
    pub guilds: Vec<Guild>,
    pub events: Vec<WorldEventState>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StarterRpgMmorpgConfig {
    pub seed: u64,
    pub default_classes: Vec<CharacterClass>,
    pub starter_skills: Vec<Skill>,
    pub starter_quests: Vec<QuestTemplate>,
    pub starter_loot_table: Vec<LootTableEntry>,
    pub world_state: WorldState,
}

pub fn xp_for_next_level(level: u32) -> u64 {
    let l = level.max(1) as u64;
    100 + l * l * 25
}

pub fn apply_xp(profile: &mut CharacterProfile, gained: u64) -> u32 {
    profile.xp += gained;
    let mut level_ups = 0;

    loop {
        let needed = xp_for_next_level(profile.level);
        if profile.xp < needed {
            break;
        }
        profile.xp -= needed;
        profile.level += 1;
        level_ups += 1;
        profile.stats.hp += 18;
        profile.stats.mp += 10;
        profile.stats.attack += 3;
        profile.stats.defense += 2;
        profile.stats.spell_power += 3;
    }

    level_ups
}

pub fn resolve_combat_event(
    attacker: &CharacterProfile,
    defender: &mut CharacterProfile,
    skill: Option<&Skill>,
    seed: u64,
) -> CombatEvent {
    let mut rng = StdRng::seed_from_u64(seed);

    let base_power = if let Some(skill) = skill {
        skill.base_power + attacker.stats.spell_power / 2
    } else {
        attacker.stats.attack
    };

    let gear_bonus = (attacker.equipped_item_level as i32 / 2).max(0);
    let mut raw_damage = base_power + gear_bonus + rng.random_range(0..=8);
    let crit_roll: f32 = rng.random_range(0.0f32..1.0f32);
    let did_crit = crit_roll <= attacker.stats.crit_chance.clamp(0.0f32, 0.95f32);
    if did_crit {
        raw_damage = ((raw_damage as f32) * 1.6) as i32;
    }

    let mitigation = defender.stats.defense + rng.random_range(0..=4);
    let mitigated_damage = (raw_damage - mitigation).max(1);
    defender.stats.hp = (defender.stats.hp - mitigated_damage).max(0);

    CombatEvent {
        attacker_id: attacker.id.clone(),
        defender_id: defender.id.clone(),
        skill_id: skill.map(|s| s.id.clone()),
        raw_damage,
        mitigated_damage,
        did_crit,
        defender_hp_after: defender.stats.hp,
        defeated: defender.stats.hp == 0,
    }
}

pub fn update_quest_progress(
    progress: &mut QuestProgress,
    objective_id: &str,
    amount: u32,
    quest: &QuestTemplate,
) {
    let entry = progress
        .objective_progress
        .entry(objective_id.to_string())
        .or_insert(0);
    *entry = entry.saturating_add(amount);

    progress.completed = quest.objectives.iter().all(|obj| {
        let current = *progress.objective_progress.get(&obj.objective_id).unwrap_or(&0);
        current >= obj.required
    });
}

pub fn roll_loot(entries: &[LootTableEntry], seed: u64, drops: usize) -> Vec<LootDrop> {
    if entries.is_empty() || drops == 0 {
        return Vec::new();
    }

    let mut rng = StdRng::seed_from_u64(seed);
    let total_weight: u32 = entries.iter().map(|e| e.weight).sum();
    if total_weight == 0 {
        return Vec::new();
    }

    let mut result = Vec::new();
    for _ in 0..drops {
        let mut roll = rng.random_range(0..total_weight);
        let mut picked = &entries[0];

        for entry in entries {
            if roll < entry.weight {
                picked = entry;
                break;
            }
            roll -= entry.weight;
        }

        let qty = if picked.min_qty >= picked.max_qty {
            picked.min_qty
        } else {
            rng.random_range(picked.min_qty..=picked.max_qty)
        };

        result.push(LootDrop {
            item_id: picked.item_id.clone(),
            rarity: picked.rarity.clone(),
            quantity: qty,
        });
    }
    result
}

pub fn tick_world(world: &mut WorldState, elapsed_ms: u64) {
    world.world_time_ms = world.world_time_ms.saturating_add(elapsed_ms);
    for event in &mut world.events {
        event.active = world.world_time_ms >= event.starts_at_ms && world.world_time_ms < event.ends_at_ms;
    }
}

pub fn create_party(leader_id: &str) -> Party {
    Party {
        party_id: format!("party_{}", leader_id),
        leader: leader_id.to_string(),
        members: vec![leader_id.to_string()],
    }
}

pub fn add_member_to_party(party: &mut Party, member_id: &str) -> bool {
    if party.members.iter().any(|m| m == member_id) {
        return false;
    }
    party.members.push(member_id.to_string());
    true
}

pub fn remove_member_from_party(party: &mut Party, member_id: &str) -> bool {
    if let Some(idx) = party.members.iter().position(|m| m == member_id) {
        party.members.remove(idx);
        if party.leader == member_id
            && let Some(next) = party.members.first()
        {
            party.leader = next.clone();
        }
        true
    } else {
        false
    }
}

pub fn create_guild(name: &str, founder_id: &str) -> Guild {
    Guild {
        guild_id: format!("guild_{}", name.to_lowercase().replace(' ', "_")),
        name: name.to_string(),
        level: 1,
        members: vec![founder_id.to_string()],
        message_of_the_day: "Welcome to the guild.".to_string(),
    }
}

pub fn add_member_to_guild(guild: &mut Guild, member_id: &str) -> bool {
    if guild.members.iter().any(|m| m == member_id) {
        return false;
    }
    guild.members.push(member_id.to_string());
    true
}

pub fn remove_member_from_guild(guild: &mut Guild, member_id: &str) -> bool {
    if let Some(idx) = guild.members.iter().position(|m| m == member_id) {
        guild.members.remove(idx);
        true
    } else {
        false
    }
}

pub fn start_quest_progress(quest: &QuestTemplate) -> QuestProgress {
    let mut objective_progress = BTreeMap::new();
    for o in &quest.objectives {
        objective_progress.insert(o.objective_id.clone(), 0);
    }
    QuestProgress {
        quest_id: quest.quest_id.clone(),
        objective_progress,
        completed: false,
    }
}

pub fn can_accept_quest(profile: &CharacterProfile, quest: &QuestTemplate) -> bool {
    profile.level >= quest.min_level
}

pub fn generate_starter_rpg_mmorpg_config(seed: u64, world_name: String) -> StarterRpgMmorpgConfig {
    let starter_skills = vec![
        Skill {
            id: "skill_slash".to_string(),
            name: "Slash".to_string(),
            class: CharacterClass::Warrior,
            damage_type: DamageType::Physical,
            base_power: 28,
            mana_cost: 0,
            cooldown_ms: 1500,
        },
        Skill {
            id: "skill_firebolt".to_string(),
            name: "Firebolt".to_string(),
            class: CharacterClass::Mage,
            damage_type: DamageType::Fire,
            base_power: 34,
            mana_cost: 12,
            cooldown_ms: 1800,
        },
        Skill {
            id: "skill_shot".to_string(),
            name: "Aimed Shot".to_string(),
            class: CharacterClass::Ranger,
            damage_type: DamageType::Physical,
            base_power: 30,
            mana_cost: 6,
            cooldown_ms: 1600,
        },
    ];

    let starter_quests = vec![
        QuestTemplate {
            quest_id: "q_start_hunt".to_string(),
            title: "Culling the Wilds".to_string(),
            min_level: 1,
            objectives: vec![QuestObjective {
                objective_id: "kill_wolf".to_string(),
                description: "Defeat wolves near the village".to_string(),
                required: 6,
            }],
            reward_xp: 220,
            reward_gold: 35,
        },
        QuestTemplate {
            quest_id: "q_supply_run".to_string(),
            title: "Supply Run".to_string(),
            min_level: 1,
            objectives: vec![QuestObjective {
                objective_id: "collect_herbs".to_string(),
                description: "Collect healing herbs".to_string(),
                required: 8,
            }],
            reward_xp: 180,
            reward_gold: 25,
        },
    ];

    let starter_loot_table = vec![
        LootTableEntry {
            item_id: "gold_coin".to_string(),
            rarity: ItemRarity::Common,
            min_qty: 5,
            max_qty: 32,
            weight: 240,
        },
        LootTableEntry {
            item_id: "healing_potion".to_string(),
            rarity: ItemRarity::Uncommon,
            min_qty: 1,
            max_qty: 3,
            weight: 95,
        },
        LootTableEntry {
            item_id: "mystic_shard".to_string(),
            rarity: ItemRarity::Rare,
            min_qty: 1,
            max_qty: 2,
            weight: 32,
        },
        LootTableEntry {
            item_id: "dragon_sigil".to_string(),
            rarity: ItemRarity::Epic,
            min_qty: 1,
            max_qty: 1,
            weight: 8,
        },
    ];

    let world_state = WorldState {
        world_name,
        server_tick_ms: 100,
        world_time_ms: 0,
        max_players_per_shard: 600,
        parties: Vec::new(),
        guilds: Vec::new(),
        events: vec![
            WorldEventState {
                event_id: "evt_blood_moon".to_string(),
                title: "Blood Moon".to_string(),
                starts_at_ms: 30 * 60 * 1000,
                ends_at_ms: 45 * 60 * 1000,
                active: false,
            },
            WorldEventState {
                event_id: "evt_world_boss".to_string(),
                title: "World Boss: Iron Colossus".to_string(),
                starts_at_ms: 60 * 60 * 1000,
                ends_at_ms: 75 * 60 * 1000,
                active: false,
            },
        ],
    };

    StarterRpgMmorpgConfig {
        seed,
        default_classes: vec![
            CharacterClass::Warrior,
            CharacterClass::Ranger,
            CharacterClass::Mage,
            CharacterClass::Cleric,
            CharacterClass::Rogue,
        ],
        starter_skills,
        starter_quests,
        starter_loot_table,
        world_state,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_progression_levels_up() {
        let mut profile = CharacterProfile {
            id: "p".into(),
            name: "Player".into(),
            class: CharacterClass::Warrior,
            level: 1,
            xp: 0,
            stats: StatBlock {
                hp: 100,
                mp: 20,
                attack: 10,
                defense: 8,
                spell_power: 4,
                crit_chance: 0.1,
                haste: 0.0,
            },
            unlocked_skills: BTreeSet::new(),
            equipped_item_level: 1,
        };
        let levels = apply_xp(&mut profile, 10_000);
        assert!(levels > 0);
        assert!(profile.level > 1);
    }

    #[test]
    fn loot_roll_is_deterministic_for_seed() {
        let entries = vec![
            LootTableEntry {
                item_id: "a".into(),
                rarity: ItemRarity::Common,
                min_qty: 1,
                max_qty: 2,
                weight: 10,
            },
            LootTableEntry {
                item_id: "b".into(),
                rarity: ItemRarity::Rare,
                min_qty: 1,
                max_qty: 1,
                weight: 1,
            },
        ];
        let d1 = roll_loot(&entries, 123, 3);
        let d2 = roll_loot(&entries, 123, 3);
        assert_eq!(
            serde_json::to_string(&d1).unwrap(),
            serde_json::to_string(&d2).unwrap()
        );
    }
}
