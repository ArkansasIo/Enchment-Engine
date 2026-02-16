// Procedural Rule Editor UI Implementation Example for egui
// Integrates with procedural_rules.json data

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ProceduralRules {
    pub prefixes: Vec<String>,
    pub suffixes: Vec<String>,
    pub rarity_table: Vec<RarityEntry>,
    pub elite_bonus: EliteBonus,
    pub boss_bonus: BossBonus,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RarityEntry {
    pub rarity: String,
    pub weight: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EliteBonus {
    pub rare: f32,
    pub epic: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BossBonus {
    pub guaranteed_rarity: String,
    pub extra_rolls: u32,
}

pub struct ProceduralRuleEditorImpl {
    pub rules: Option<ProceduralRules>,
    pub status: String,
}

impl ProceduralRuleEditorImpl {
    pub fn new() -> Self {
        let rules = Self::load_rules();
        Self { rules, status: "Ready".to_string() }
    }
    pub fn load_rules() -> Option<ProceduralRules> {
        let data = fs::read_to_string("docs/assets/procedural_rules.json").ok()?;
        serde_json::from_str(&data).ok()
    }
    pub fn save_rules(&self) {
        if let Some(rules) = &self.rules {
            let data = serde_json::to_string_pretty(rules).unwrap();
            fs::write("docs/assets/procedural_rules.json", data).ok();
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(rules) = &mut self.rules {
                ui.heading("Procedural Rules");
                ui.label("Prefixes:");
                for (i, prefix) in rules.prefixes.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(prefix);
                        if ui.button("Delete").clicked() {
                            rules.prefixes.remove(i);
                        }
                    });
                }
                if ui.button("+ Add Prefix").clicked() {
                    rules.prefixes.push("new_prefix".to_string());
                }
                ui.separator();
                ui.label("Suffixes:");
                for (i, suffix) in rules.suffixes.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(suffix);
                        if ui.button("Delete").clicked() {
                            rules.suffixes.remove(i);
                        }
                    });
                }
                if ui.button("+ Add Suffix").clicked() {
                    rules.suffixes.push("new_suffix".to_string());
                }
                ui.separator();
                ui.label("Rarity Table:");
                for (i, entry) in rules.rarity_table.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut entry.rarity);
                        ui.add(egui::DragValue::new(&mut entry.weight).clamp_range(0.0..=100.0));
                        if ui.button("Delete").clicked() {
                            rules.rarity_table.remove(i);
                        }
                    });
                }
                if ui.button("+ Add Rarity").clicked() {
                    rules.rarity_table.push(RarityEntry { rarity: "new_rarity".to_string(), weight: 1.0 });
                }
                ui.separator();
                ui.label("Elite Bonus:");
                ui.horizontal(|ui| {
                    ui.label("Rare:");
                    ui.add(egui::DragValue::new(&mut rules.elite_bonus.rare).clamp_range(0.0..=10.0));
                    ui.label("Epic:");
                    ui.add(egui::DragValue::new(&mut rules.elite_bonus.epic).clamp_range(0.0..=10.0));
                });
                ui.separator();
                ui.label("Boss Bonus:");
                ui.horizontal(|ui| {
                    ui.label("Guaranteed Rarity:");
                    ui.text_edit_singleline(&mut rules.boss_bonus.guaranteed_rarity);
                    ui.label("Extra Rolls:");
                    ui.add(egui::DragValue::new(&mut rules.boss_bonus.extra_rolls).clamp_range(0..=10));
                });
                if ui.button("Save").clicked() {
                    self.save_rules();
                    self.status = "Saved!".to_string();
                }
                ui.label(&self.status);
            } else {
                ui.label("No procedural rules loaded.");
            }
        });
    }
}
