// Building Editor UI Implementation Example for egui
// Integrates with buildings.json data (to be created)

use egui::*;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub footprint_w: u32,
    pub footprint_h: u32,
    pub tier: u32,
    pub biome: String,
    pub cost: u32,
    pub hp: u32,
    pub defense: u32,
}

pub struct BuildingEditorImpl {
    pub buildings: Vec<Building>,
    pub selected: Option<usize>,
    pub status: String,
}

impl BuildingEditorImpl {
    pub fn new() -> Self {
        let buildings = Self::load_buildings();
        Self { buildings, selected: None, status: "Ready".to_string() }
    }
    pub fn load_buildings() -> Vec<Building> {
        let data = fs::read_to_string("docs/assets/buildings.json").unwrap_or_default();
        serde_json::from_str(&data).unwrap_or_default()
    }
    pub fn save_buildings(&self) {
        let data = serde_json::to_string_pretty(&self.buildings).unwrap();
        fs::write("docs/assets/buildings.json", data).ok();
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("building_list").show(ctx, |ui| {
            ui.heading("Buildings");
            for (i, b) in self.buildings.iter().enumerate() {
                if ui.selectable_label(self.selected == Some(i), &b.name).clicked() {
                    self.selected = Some(i);
                }
            }
            if ui.button("+ Add Building").clicked() {
                self.buildings.push(Building {
                    id: "new_building".to_string(),
                    name: "New Building".to_string(),
                    footprint_w: 3,
                    footprint_h: 3,
                    tier: 1,
                    biome: "forest".to_string(),
                    cost: 100,
                    hp: 500,
                    defense: 10,
                });
                self.selected = Some(self.buildings.len() - 1);
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(idx) = self.selected {
                let b = &mut self.buildings[idx];
                ui.heading(&b.name);
                ui.horizontal(|ui| {
                    ui.label("ID:");
                    ui.text_edit_singleline(&mut b.id);
                    ui.label("Biome:");
                    ui.text_edit_singleline(&mut b.biome);
                });
                ui.horizontal(|ui| {
                    ui.label("Footprint W:");
                    ui.add(egui::DragValue::new(&mut b.footprint_w).clamp_range(1..=32));
                    ui.label("Footprint H:");
                    ui.add(egui::DragValue::new(&mut b.footprint_h).clamp_range(1..=32));
                });
                ui.horizontal(|ui| {
                    ui.label("Tier:");
                    ui.add(egui::DragValue::new(&mut b.tier).clamp_range(1..=5));
                    ui.label("Cost:");
                    ui.add(egui::DragValue::new(&mut b.cost).clamp_range(0..=99999));
                });
                ui.horizontal(|ui| {
                    ui.label("HP:");
                    ui.add(egui::DragValue::new(&mut b.hp).clamp_range(0..=99999));
                    ui.label("Defense:");
                    ui.add(egui::DragValue::new(&mut b.defense).clamp_range(0..=9999));
                });
                if ui.button("Save").clicked() {
                    self.save_buildings();
                    self.status = "Saved!".to_string();
                }
                if ui.button("Delete").clicked() {
                    self.buildings.remove(idx);
                    self.selected = None;
                }
                ui.label(&self.status);
            } else {
                ui.label("Select a building to edit.");
            }
        });
    }
}
