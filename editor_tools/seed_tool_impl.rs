// Seed Tool Implementation for Map Generation and Procedural Systems
// Provides UI for setting, randomizing, and copying seeds for map/dungeon/loot generation

use egui::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

pub struct SeedToolImpl {
    pub seed: u64,
    pub status: String,
}

impl SeedToolImpl {
    pub fn new() -> Self {
        Self {
            seed: rand::thread_rng().gen(),
            status: "Ready".to_string(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Seed Tool");
            ui.horizontal(|ui| {
                ui.label("Seed:");
                let mut seed_str = self.seed.to_string();
                if ui.text_edit_singleline(&mut seed_str).changed() {
                    if let Ok(val) = seed_str.parse() {
                        self.seed = val;
                        self.status = "Seed updated".to_string();
                    } else {
                        self.status = "Invalid seed value".to_string();
                    }
                }
                if ui.button("Randomize").clicked() {
                    self.seed = rand::thread_rng().gen();
                    self.status = "Seed randomized".to_string();
                }
                if ui.button("Copy").clicked() {
                    ui.output_mut(|o| o.copied_text = self.seed.to_string());
                    self.status = "Seed copied to clipboard".to_string();
                }
            });
            ui.label(&self.status);
            ui.separator();
            ui.label("Use this seed for map, dungeon, loot, or procedural generation to ensure reproducibility.");
        });
    }

    pub fn get_rng(&self) -> StdRng {
        StdRng::seed_from_u64(self.seed)
    }
}
