//! Editor bottom panel: Terminal, Debug Console, Output, Problems, and Log.

use egui::*;

pub struct BottomPanelState {
    pub terminal_log: Vec<String>,
    pub debug_log: Vec<String>,
    pub output_log: Vec<String>,
    pub problems_log: Vec<String>,
    pub active_tab: BottomTab,
    pub input_cmd: String,
}

pub enum BottomTab {
    Terminal,
    Debug,
    Output,
    Problems,
    Log,
}

impl BottomPanelState {
    pub fn new() -> Self {
        Self {
            terminal_log: Vec::new(),
            debug_log: Vec::new(),
            output_log: Vec::new(),
            problems_log: Vec::new(),
            active_tab: BottomTab::Terminal,
            input_cmd: String::new(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.selectable_label(matches!(self.active_tab, BottomTab::Terminal), "Terminal").clicked() {
                    self.active_tab = BottomTab::Terminal;
                }
                if ui.selectable_label(matches!(self.active_tab, BottomTab::Debug), "Debug Console").clicked() {
                    self.active_tab = BottomTab::Debug;
                }
                if ui.selectable_label(matches!(self.active_tab, BottomTab::Output), "Output").clicked() {
                    self.active_tab = BottomTab::Output;
                }
                if ui.selectable_label(matches!(self.active_tab, BottomTab::Problems), "Problems").clicked() {
                    self.active_tab = BottomTab::Problems;
                }
                if ui.selectable_label(matches!(self.active_tab, BottomTab::Log), "Log").clicked() {
                    self.active_tab = BottomTab::Log;
                }
            });
            ui.separator();
            match self.active_tab {
                BottomTab::Terminal => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.terminal_log {
                            ui.label(line);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("> ");
                        ui.text_edit_singleline(&mut self.input_cmd);
                        if ui.button("Run").clicked() {
                            // TODO: Execute command and append result to terminal_log
                            self.terminal_log.push(format!("$ {}", self.input_cmd));
                            self.input_cmd.clear();
                        }
                    });
                }
                BottomTab::Debug => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.debug_log {
                            ui.label(line);
                        }
                    });
                }
                BottomTab::Output => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.output_log {
                            ui.label(line);
                        }
                    });
                }
                BottomTab::Problems => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for line in &self.problems_log {
                            ui.label(line);
                        }
                    });
                }
                BottomTab::Log => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // TODO: Integrate with application log
                        ui.label("Application log output...");
                    });
                }
            }
        });
    }
}
