// Main Editor UI Layout for Enchentment Engine
// Uses egui for GUI rendering

use egui::*;
use once_cell::sync::Lazy;
use i18n_embed::fluent::FluentLanguageLoader;
use i18n_embed::unic_langid::LanguageIdentifier;
use crate::creator::i18n;

pub enum Tool {
    AssetBrowser,
    MonsterEditor,
    ItemEditor,
    LootTableEditor,
    BuildingEditor,
    ProceduralRuleEditor,
    TilemapTool,
    AnimationEditor,
    StatCurveVisualizer,
    SaveExport,
    SeedTool,
}

pub struct MainEditor {
    pub active_tool: Tool,
    pub asset_browser: super::asset_browser::AssetBrowser,
    pub monster_editor: super::monster_editor::MonsterEditor,
    pub item_editor: super::item_editor::ItemEditor,
    pub loot_table_editor: super::loot_table_editor::LootTableEditor,
    pub building_editor: super::building_editor::BuildingEditor,
    pub procedural_rule_editor: super::procedural_rule_editor::ProceduralRuleEditor,
    pub tilemap_tool: super::tilemap_tool::TilemapTool,
    pub animation_editor: super::animation_editor::AnimationEditor,
    pub stat_curve_visualizer: super::stat_curve_visualizer::StatCurveVisualizer,
    pub save_export: super::save_export::SaveExportTool,
    pub seed_tool: super::seed_tool_impl::SeedToolImpl,
    pub status: String,
    pub show_settings: bool,
    pub project_name: String,
    pub user_name: String,
}

static LANGUAGE_LOADER: Lazy<FluentLanguageLoader> = Lazy::new(|| {
    let loader = i18n::LANGUAGE_LOADER.clone();
    let _ = i18n::select_system_locales();
    loader
});

impl MainEditor {
    pub fn new() -> Self {
        Self {
            active_tool: Tool::AssetBrowser,
            asset_browser: super::asset_browser::AssetBrowser::new(),
            monster_editor: super::monster_editor::MonsterEditor::new(),
            item_editor: super::item_editor::ItemEditor::new(),
            loot_table_editor: super::loot_table_editor::LootTableEditor::new(),
            building_editor: super::building_editor::BuildingEditor::new(),
            procedural_rule_editor: super::procedural_rule_editor::ProceduralRuleEditor::new(),
            tilemap_tool: super::tilemap_tool::TilemapTool::new(),
            animation_editor: super::animation_editor::AnimationEditor::new(),
            stat_curve_visualizer: super::stat_curve_visualizer::StatCurveVisualizer::new(),
            save_export: super::save_export::SaveExportTool::new(),
            seed_tool: super::seed_tool_impl::SeedToolImpl::new(),
            status: "Ready".to_string(),
            show_settings: false,
            project_name: "MyGame".to_string(),
            user_name: "Player".to_string(),
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        // Theming
        let mut style = ctx.style().clone();
        style.visuals = egui::Visuals::dark();
        ctx.set_style(style);

        TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(LANGUAGE_LOADER.lookup("info_welcome").unwrap_or("Enchentment Engine Editor"));
                ui.separator();
                if ui.button(LANGUAGE_LOADER.lookup("menu_undo").unwrap_or("↶ Undo")).clicked() {
                    // TODO: Call undo logic (integrate with UndoManager)
                    self.status = "Undo action (not yet implemented)".to_string();
                }
                if ui.button(LANGUAGE_LOADER.lookup("menu_redo").unwrap_or("↷ Redo")).clicked() {
                    // TODO: Call redo logic (integrate with UndoManager)
                    self.status = "Redo action (not yet implemented)".to_string();
                }
                ui.separator();
                ui.label(format!("Project: {}", self.project_name));
                if ui.button("⚙️ Settings").clicked() {
                    self.show_settings = true;
                }
                // ... file menu, quick actions, theme switcher
            });
        });

        if self.show_settings {
            egui::Window::new("Settings").open(&mut self.show_settings).show(ctx, |ui| {
                ui.heading("User & Project Settings");
                ui.horizontal(|ui| {
                    ui.label("Project Name:");
                    ui.text_edit_singleline(&mut self.project_name);
                });
                ui.horizontal(|ui| {
                    ui.label("User Name:");
                    ui.text_edit_singleline(&mut self.user_name);
                });
                ui.label("(Add more settings as needed)");
                if ui.button("Close").clicked() {
                    self.show_settings = false;
                }
            });
        }

        SidePanel::left("sidebar").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button(LANGUAGE_LOADER.lookup("asset_browser").unwrap_or("📦 Asset Browser")).clicked() { self.active_tool = Tool::AssetBrowser; }
                if ui.button(LANGUAGE_LOADER.lookup("monster_editor").unwrap_or("🐲 Monster Editor")).clicked() { self.active_tool = Tool::MonsterEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("item_editor").unwrap_or("🗡️ Item Editor")).clicked() { self.active_tool = Tool::ItemEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("loot_table_editor").unwrap_or("🎲 Loot Table Editor")).clicked() { self.active_tool = Tool::LootTableEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("building_editor").unwrap_or("🏰 Building Editor")).clicked() { self.active_tool = Tool::BuildingEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("procedural_rule_editor").unwrap_or("⚙️ Procedural Rule Editor")).clicked() { self.active_tool = Tool::ProceduralRuleEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("tilemap_tool").unwrap_or("🗺️ Tilemap Tool")).clicked() { self.active_tool = Tool::TilemapTool; }
                if ui.button(LANGUAGE_LOADER.lookup("animation_editor").unwrap_or("🎞️ Animation Editor")).clicked() { self.active_tool = Tool::AnimationEditor; }
                if ui.button(LANGUAGE_LOADER.lookup("stat_curve_visualizer").unwrap_or("📈 Stat Curve Visualizer")).clicked() { self.active_tool = Tool::StatCurveVisualizer; }
                if ui.button(LANGUAGE_LOADER.lookup("save_export").unwrap_or("💾 Save/Export")).clicked() { self.active_tool = Tool::SaveExport; }
                if ui.button(LANGUAGE_LOADER.lookup("seed_tool").unwrap_or("🔑 Seed Tool")).clicked() { self.active_tool = Tool::SeedTool; }
            });
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.active_tool {
                Tool::AssetBrowser => self.asset_browser.ui(ctx),
                Tool::MonsterEditor => self.monster_editor.ui(ctx),
                Tool::ItemEditor => self.item_editor.ui(ctx),
                Tool::LootTableEditor => self.loot_table_editor.ui(ctx),
                Tool::BuildingEditor => self.building_editor.ui(ctx),
                Tool::ProceduralRuleEditor => self.procedural_rule_editor.ui(ctx),
                Tool::TilemapTool => self.tilemap_tool.ui(ctx),
                Tool::AnimationEditor => self.animation_editor.ui(ctx),
                Tool::StatCurveVisualizer => self.stat_curve_visualizer.ui(ctx),
                Tool::SaveExport => self.save_export.ui(ctx),
                Tool::SeedTool => self.seed_tool.ui(ctx),
            }
        });

        TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
                // ... show save status, errors, tooltips
            });
        });
    }
}
