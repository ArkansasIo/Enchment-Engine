// Monster Editor UI Implementation Example for egui
// Integrates with monsters.json data

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Monster {
    pub id: String,
    pub r#type: String,
    pub biome: String,
    pub level: u32,
    pub hp: u32,
    pub atk: u32,
    pub def: u32,
    pub loot_table: String,
}

pub struct MonsterEditorImpl {
    pub monsters: Vec<Monster>,
    pub selected: Option<usize>,
    pub status: String,
    pub edit: Monster,
    pub editing: bool,
}

impl MonsterEditorImpl {
    pub fn new() -> Self {
        let monsters = Self::load_monsters();
        Self {
            monsters,
            selected: None,
            status: "Ready".to_string(),
            edit: Monster::default(),
            editing: false,
        }
    }
    pub fn load_monsters() -> Vec<Monster> {
        let data = fs::read_to_string("docs/assets/monsters.json").unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }
    pub fn save_monsters(&self) {
        let data = serde_json::to_string_pretty(&self.monsters).unwrap();
        fs::write("docs/assets/monsters.json", data).ok();
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("monster_list").show(ctx, |ui| {
            ui.heading("Monsters");
            for (i, m) in self.monsters.iter().enumerate() {
                if ui.selectable_label(self.selected == Some(i), &m.id).clicked() {
                    self.selected = Some(i);
                    self.edit = m.clone();
                    self.editing = false;
                }
            }
            if ui.button("+ Add Monster").clicked() {
                self.edit = Monster::default();
                self.editing = true;
                self.selected = None;
            }
            if let Some(idx) = self.selected {
                if ui.button("Delete").clicked() {
                    self.monsters.remove(idx);
                    self.selected = None;
                    self.status = "Monster deleted".to_string();
                    self.save_monsters();
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Monster Editor");
            ui.label(&self.status);
            if self.editing || self.selected.is_some() {
                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut self.edit.id);
                });
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.text_edit_singleline(&mut self.edit.r#type);
                });
                ui.horizontal(|ui| {
                    ui.label("Biome:");
                    ui.text_edit_singleline(&mut self.edit.biome);
                });
                ui.horizontal(|ui| {
                    ui.label("Level:");
                    ui.add(egui::DragValue::new(&mut self.edit.level));
                });
                ui.horizontal(|ui| {
                    ui.label("HP:");
                    ui.add(egui::DragValue::new(&mut self.edit.hp));
                });
                ui.horizontal(|ui| {
                    ui.label("ATK:");
                    ui.add(egui::DragValue::new(&mut self.edit.atk));
                });
                ui.horizontal(|ui| {
                    ui.label("DEF:");
                    ui.add(egui::DragValue::new(&mut self.edit.def));
                });
                ui.horizontal(|ui| {
                    ui.label("Loot Table:");
                    ui.text_edit_singleline(&mut self.edit.loot_table);
                });
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if self.editing {
                            self.monsters.push(self.edit.clone());
                            self.status = "Monster added".to_string();
                        } else if let Some(idx) = self.selected {
                            self.monsters[idx] = self.edit.clone();
                            self.status = "Monster updated".to_string();
                        }
                        self.save_monsters();
                        self.editing = false;
                        self.selected = None;
                    }
                    if ui.button("Cancel").clicked() {
                        self.editing = false;
                        self.selected = None;
                    }
                });
            }
        });
    }
}
