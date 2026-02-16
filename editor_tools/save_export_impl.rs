// Save/Export Tool UI Implementation Example for egui
// (Stub: extend with file dialogs, import/export, etc.)

use egui::*;

pub struct SaveExportImpl {
    pub status: String,
    pub last_path: Option<std::path::PathBuf>,
}

impl SaveExportImpl {
    pub fn new() -> Self {
        Self {
            status: "Ready".to_string(),
            last_path: None,
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Save/Export Tool");
            ui.label("Save edits, import/export assets here.");
            ui.horizontal(|ui| {
                if ui.button("Save Project").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_title("Save Project As").save_file() {
                        // TODO: Serialize and save all project data here
                        self.last_path = Some(path.clone());
                        self.status = format!("Project saved to {}", path.display());
                    }
                }
                if ui.button("Export Data").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_title("Export Data").save_file() {
                        // TODO: Export all data types (assets, monsters, items, loot, rules, etc.)
                        self.last_path = Some(path.clone());
                        self.status = format!("Data exported to {}", path.display());
                    }
                }
                if ui.button("Import Data").clicked() {
                    if let Some(path) = rfd::FileDialog::new().set_title("Import Data").pick_file() {
                        // TODO: Import and merge data from file
                        self.last_path = Some(path.clone());
                        self.status = format!("Data imported from {}", path.display());
                    }
                }
            });
            ui.label(&self.status);
        });
    }
}
