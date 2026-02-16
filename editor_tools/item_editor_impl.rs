// Item Editor UI Implementation Example for egui
// Integrates with items.json data

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Item {
    pub id: String,
    pub r#type: String,
    pub rarity: String,
    pub price: u32,
    pub stack_max: u32,
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
                    ui.label("Type:");
                    ui.text_edit_singleline(&mut self.edit.r#type);
                });
                ui.horizontal(|ui| {
                    ui.label("Rarity:");
                    ui.text_edit_singleline(&mut self.edit.rarity);
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
