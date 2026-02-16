// Loot Table Editor UI Implementation Example for egui
// Integrates with loot_tables.json data

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LootEntry {
    pub item_id: String,
    pub weight: u32,
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct LootTable {
    pub loot_table_id: String,
    pub entries: Vec<LootEntry>,
}

pub struct LootTableEditorImpl {
    pub tables: Vec<LootTable>,
    pub selected: Option<usize>,
    pub status: String,
    pub edit: LootTable,
    pub editing: bool,
    pub entry_edit: LootEntry,
    pub entry_editing: Option<usize>,
}

impl LootTableEditorImpl {
    pub fn new() -> Self {
        let tables = Self::load_tables();
        Self {
            tables,
            selected: None,
            status: "Ready".to_string(),
            edit: LootTable::default(),
            editing: false,
            entry_edit: LootEntry::default(),
            entry_editing: None,
        }
    }
    pub fn load_tables() -> Vec<LootTable> {
        let data = fs::read_to_string("docs/assets/loot_tables.json").unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }
    pub fn save_tables(&self) {
        let data = serde_json::to_string_pretty(&self.tables).unwrap();
        fs::write("docs/assets/loot_tables.json", data).ok();
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("loot_table_list").show(ctx, |ui| {
            ui.heading("Loot Tables");
            for (i, t) in self.tables.iter().enumerate() {
                if ui.selectable_label(self.selected == Some(i), &t.loot_table_id).clicked() {
                    self.selected = Some(i);
                    self.edit = t.clone();
                    self.editing = false;
                }
            }
            if ui.button("+ Add Table").clicked() {
                self.edit = LootTable::default();
                self.editing = true;
                self.selected = None;
            }
            if let Some(idx) = self.selected {
                if ui.button("Delete").clicked() {
                    self.tables.remove(idx);
                    self.selected = None;
                    self.status = "Table deleted".to_string();
                    self.save_tables();
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Loot Table Editor");
            ui.label(&self.status);
            if self.editing || self.selected.is_some() {
                ui.horizontal(|ui| {
                    ui.label("Table ID:");
                    ui.text_edit_singleline(&mut self.edit.loot_table_id);
                });
                ui.separator();
                ui.label("Entries:");
                for (i, entry) in self.edit.entries.iter().enumerate() {
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ({}-{}, w={})", entry.item_id, entry.min, entry.max, entry.weight));
                        if ui.button("Edit").clicked() {
                            self.entry_edit = entry.clone();
                            self.entry_editing = Some(i);
                        }
                        if ui.button("Remove").clicked() {
                            self.edit.entries.remove(i);
                        }
                    });
                }
                if ui.button("+ Add Entry").clicked() {
                    self.entry_edit = LootEntry::default();
                    self.entry_editing = None;
                }
                if self.entry_editing.is_some() || self.entry_edit.item_id != "" {
                    ui.separator();
                    ui.label("Edit Entry:");
                    ui.horizontal(|ui| {
                        ui.label("Item ID:");
                        ui.text_edit_singleline(&mut self.entry_edit.item_id);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Weight:");
                        ui.add(egui::DragValue::new(&mut self.entry_edit.weight));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Min:");
                        ui.add(egui::DragValue::new(&mut self.entry_edit.min));
                        ui.label("Max:");
                        ui.add(egui::DragValue::new(&mut self.entry_edit.max));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Save Entry").clicked() {
                            if let Some(idx) = self.entry_editing {
                                self.edit.entries[idx] = self.entry_edit.clone();
                            } else {
                                self.edit.entries.push(self.entry_edit.clone());
                            }
                            self.entry_edit = LootEntry::default();
                            self.entry_editing = None;
                        }
                        if ui.button("Cancel").clicked() {
                            self.entry_edit = LootEntry::default();
                            self.entry_editing = None;
                        }
                    });
                }
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save Table").clicked() {
                        if self.editing {
                            self.tables.push(self.edit.clone());
                            self.status = "Table added".to_string();
                        } else if let Some(idx) = self.selected {
                            self.tables[idx] = self.edit.clone();
                            self.status = "Table updated".to_string();
                        }
                        self.save_tables();
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
