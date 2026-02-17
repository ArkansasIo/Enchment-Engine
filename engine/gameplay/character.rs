//! Character: advanced pawn with movement/animation.


use crate::engine::items::{Item, Buff, Debuff};

#[derive(Debug, Clone, Default)]
pub struct Character {
    pub id: u64,
    pub name: String,
    pub class: Option<String>,
    pub race: Option<String>,
    pub background: Option<String>,
    pub level: u32,
    pub xp: u64,
    pub stats: CharacterStats,
    pub inventory: Vec<Item>,
    pub equipped: Vec<Item>,
    pub buffs: Vec<Buff>,
    pub debuffs: Vec<Debuff>,
}

#[derive(Debug, Clone, Default)]
pub struct CharacterStats {
    pub str_score: i32,
    pub dex_score: i32,
    pub con_score: i32,
    pub int_score: i32,
    pub wis_score: i32,
    pub cha_score: i32,
    pub ac: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub speed: i32,
    pub initiative: i32,
    pub prof_bonus: i32,
    pub resistances: Vec<String>,
    pub vulnerabilities: Vec<String>,
    pub immunities: Vec<String>,
}


impl Character {
    pub fn new(id: u64, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            class: None,
            race: None,
            background: None,
            level: 1,
            xp: 0,
            stats: CharacterStats::default(),
            inventory: vec![],
            equipped: vec![],
            buffs: vec![],
            debuffs: vec![],
        }
    }

    pub fn apply_buff(&mut self, buff: Buff) {
        self.buffs.push(buff);
        self.recalculate_stats();
    }

    pub fn remove_buff(&mut self, buff_id: &str) {
        self.buffs.retain(|b| b.id != buff_id);
        self.recalculate_stats();
    }

    pub fn apply_debuff(&mut self, debuff: Debuff) {
        self.debuffs.push(debuff);
        self.recalculate_stats();
    }

    pub fn remove_debuff(&mut self, debuff_id: &str) {
        self.debuffs.retain(|d| d.id != debuff_id);
        self.recalculate_stats();
    }

    pub fn equip_item(&mut self, item: Item) {
        self.equipped.push(item);
        self.recalculate_stats();
    }

    pub fn unequip_item(&mut self, item_id: &str) {
        self.equipped.retain(|i| i.id != item_id);
        self.recalculate_stats();
    }

    pub fn recalculate_stats(&mut self) {
        // TODO: Implement stat calculation from base, equipment, buffs, debuffs
        // This is a placeholder for full DnD 5e/MMORPG stat logic
    }
}
