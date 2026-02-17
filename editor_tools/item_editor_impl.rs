// Item Editor UI Implementation Example for egui
// Integrates with items.json data

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: String,
    pub rarity: String,
    pub attunement: Option<bool>,
    pub magic: Option<bool>,
    pub description: Option<String>,
    pub class_restriction: Option<Vec<String>>,
    pub race_restriction: Option<Vec<String>>,
    pub proficiency_required: Option<Vec<String>>,
    pub stats: Option<ItemStats>,
    pub price: u32,
    pub stack_max: u32,
    pub features: Option<Vec<String>>,
    pub traits: Option<Vec<String>>,
    pub spell_slots: Option<u8>,
    pub charges: Option<u32>,
    pub uses: Option<u32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
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
}

pub struct ItemEditorImpl {
    pub items: Vec<Item>,
    pub selected: Option<usize>,
    pub status: String,
    pub edit: Item,
    pub editing: bool,
}

impl ItemEditorImpl {
    pub fn new() -> Self {
        let items = Self::load_items();
        Self {
            items,
            selected: None,
            status: "Ready".to_string(),
            edit: Item::default(),
            editing: false,
        }
    }
    pub fn load_items() -> Vec<Item> {
        let data = fs::read_to_string("docs/assets/items.json").unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }
    pub fn save_items(&self) {
        let data = serde_json::to_string_pretty(&self.items).unwrap();
        fs::write("docs/assets/items.json", data).ok();
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("item_list").show(ctx, |ui| {
            ui.heading("Items");
            for (i, item) in self.items.iter().enumerate() {
                if ui.selectable_label(self.selected == Some(i), &item.id).clicked() {
                    self.selected = Some(i);
                    self.edit = item.clone();
                    self.editing = false;
                }
            }
            if ui.button("+ Add Item").clicked() {
                self.edit = Item::default();
                self.editing = true;
                self.selected = None;
            }
            if let Some(idx) = self.selected {
                if ui.button("Delete").clicked() {
                    self.items.remove(idx);
                    self.selected = None;
                    self.status = "Item deleted".to_string();
                    self.save_items();
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Item Editor");
            ui.label(&self.status);
            if self.editing || self.selected.is_some() {
                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut self.edit.id);
                });
                ui.horizontal(|ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.edit.name);
                });
                ui.horizontal(|ui| {
                    ui.label("Type:");
                    ui.text_edit_singleline(&mut self.edit.item_type);
                });
                ui.horizontal(|ui| {
                    ui.label("Rarity:");
                    ui.text_edit_singleline(&mut self.edit.rarity);
                });
                ui.horizontal(|ui| {
                    ui.label("Attunement:");
                    let mut attune = self.edit.attunement.unwrap_or(false);
                    if ui.checkbox(&mut attune, "Required").changed() {
                        self.edit.attunement = Some(attune);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Magic:");
                    let mut magic = self.edit.magic.unwrap_or(false);
                    if ui.checkbox(&mut magic, "Magical").changed() {
                        self.edit.magic = Some(magic);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Description:");
                    if self.edit.description.is_none() { self.edit.description = Some(String::new()); }
                    ui.text_edit_multiline(self.edit.description.as_mut().unwrap());
                });
                ui.horizontal(|ui| {
                    ui.label("Class Restriction:");
                    if self.edit.class_restriction.is_none() { self.edit.class_restriction = Some(vec![]); }
                    let text = self.edit.class_restriction.as_ref().unwrap().join(", ");
                    let mut buf = text.clone();
                    if ui.text_edit_singleline(&mut buf).changed() {
                        self.edit.class_restriction = Some(buf.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Race Restriction:");
                    if self.edit.race_restriction.is_none() { self.edit.race_restriction = Some(vec![]); }
                    let text = self.edit.race_restriction.as_ref().unwrap().join(", ");
                    let mut buf = text.clone();
                    if ui.text_edit_singleline(&mut buf).changed() {
                        self.edit.race_restriction = Some(buf.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Proficiency Required:");
                    if self.edit.proficiency_required.is_none() { self.edit.proficiency_required = Some(vec![]); }
                    let text = self.edit.proficiency_required.as_ref().unwrap().join(", ");
                    let mut buf = text.clone();
                    if ui.text_edit_singleline(&mut buf).changed() {
                        self.edit.proficiency_required = Some(buf.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Price:");
                    ui.add(egui::DragValue::new(&mut self.edit.price));
                });
                ui.horizontal(|ui| {
                    ui.label("Stack Max:");
                    ui.add(egui::DragValue::new(&mut self.edit.stack_max));
                });
                ui.horizontal(|ui| {
                    ui.label("Features:");
                    if self.edit.features.is_none() { self.edit.features = Some(vec![]); }
                    let text = self.edit.features.as_ref().unwrap().join(", ");
                    let mut buf = text.clone();
                    if ui.text_edit_singleline(&mut buf).changed() {
                        self.edit.features = Some(buf.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Traits:");
                    if self.edit.traits.is_none() { self.edit.traits = Some(vec![]); }
                    let text = self.edit.traits.as_ref().unwrap().join(", ");
                    let mut buf = text.clone();
                    if ui.text_edit_singleline(&mut buf).changed() {
                        self.edit.traits = Some(buf.split(",").map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Spell Slots:");
                    let mut slots = self.edit.spell_slots.unwrap_or(0);
                    if ui.add(egui::DragValue::new(&mut slots)).changed() {
                        self.edit.spell_slots = Some(slots);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Charges:");
                    let mut charges = self.edit.charges.unwrap_or(0);
                    if ui.add(egui::DragValue::new(&mut charges)).changed() {
                        self.edit.charges = Some(charges);
                    }
                });
                ui.horizontal(|ui| {
                    ui.label("Uses:");
                    let mut uses = self.edit.uses.unwrap_or(0);
                    if ui.add(egui::DragValue::new(&mut uses)).changed() {
                        self.edit.uses = Some(uses);
                    }
                });
                // ItemStats
                if self.edit.stats.is_none() { self.edit.stats = Some(ItemStats::default()); }
                let stats = self.edit.stats.as_mut().unwrap();
                ui.collapsing("Stats", |ui| {
                    ui.horizontal(|ui| { ui.label("STR:"); ui.add(egui::DragValue::new(&mut stats.str_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("DEX:"); ui.add(egui::DragValue::new(&mut stats.dex_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("CON:"); ui.add(egui::DragValue::new(&mut stats.con_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("INT:"); ui.add(egui::DragValue::new(&mut stats.int_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("WIS:"); ui.add(egui::DragValue::new(&mut stats.wis_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("CHA:"); ui.add(egui::DragValue::new(&mut stats.cha_mod.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("AC:"); ui.add(egui::DragValue::new(&mut stats.ac.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("HP:"); ui.add(egui::DragValue::new(&mut stats.hp.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("Speed:"); ui.add(egui::DragValue::new(&mut stats.speed.get_or_insert(0))); });
                    ui.horizontal(|ui| { ui.label("Initiative:"); ui.add(egui::DragValue::new(&mut stats.initiative.get_or_insert(0))); });
                });
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if self.editing {
                            self.items.push(self.edit.clone());
                            self.status = "Item added".to_string();
                        } else if let Some(idx) = self.selected {
                            self.items[idx] = self.edit.clone();
                            self.status = "Item updated".to_string();
                        }
                        self.save_items();
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
