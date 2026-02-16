// Asset Browser UI Implementation with Import/Export for egui
// Handles image and JSON asset import/export

use egui::*;
use std::fs;
use std::path::PathBuf;

#[derive(Default)]
pub struct Asset {
    pub name: String,
    pub path: PathBuf,
}

pub struct AssetBrowserImpl {
    pub assets: Vec<Asset>,
    pub selected: Option<usize>,
    pub status: String,
    pub import_path: String,
    pub export_path: String,
}

impl AssetBrowserImpl {
    pub fn new() -> Self {
        let assets = Self::load_assets();
        Self {
            assets,
            selected: None,
            status: "Ready".to_string(),
            import_path: String::new(),
            export_path: String::new(),
        }
    }
    pub fn load_assets() -> Vec<Asset> {
        let mut assets = Vec::new();
        if let Ok(entries) = fs::read_dir("assets/") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                        assets.push(Asset {
                            name: name.to_string(),
                            path: path.clone(),
                        });
                    }
                }
            }
        }
        assets
    }
    pub fn import_asset(&mut self, file_path: &str) {
        let src = PathBuf::from(file_path);
        if let Some(name) = src.file_name().and_then(|n| n.to_str()) {
            let dest = PathBuf::from("assets/").join(name);
            if let Err(e) = fs::copy(&src, &dest) {
                self.status = format!("Import failed: {}", e);
            } else {
                self.status = format!("Imported {}", name);
                self.assets = Self::load_assets();
            }
        }
    }
    pub fn export_asset(&mut self, idx: usize, dest_path: &str) {
        if let Some(asset) = self.assets.get(idx) {
            if let Err(e) = fs::copy(&asset.path, dest_path) {
                self.status = format!("Export failed: {}", e);
            } else {
                self.status = format!("Exported {}", asset.name);
            }
        }
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("asset_list").show(ctx, |ui| {
            ui.heading("Assets");
            for (i, asset) in self.assets.iter().enumerate() {
                if ui.selectable_label(self.selected == Some(i), &asset.name).clicked() {
                    self.selected = Some(i);
                }
            }
            if ui.button("Reload").clicked() {
                self.assets = Self::load_assets();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Asset Browser");
            ui.label(&self.status);
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Import file:");
                ui.text_edit_singleline(&mut self.import_path);
                if ui.button("Import").clicked() {
                    self.import_asset(&self.import_path);
                }
            });
            if let Some(idx) = self.selected {
                ui.horizontal(|ui| {
                    ui.label("Export to:");
                    ui.text_edit_singleline(&mut self.export_path);
                    if ui.button("Export").clicked() {
                        self.export_asset(idx, &self.export_path);
                    }
                });
            }
        });
    }
}
