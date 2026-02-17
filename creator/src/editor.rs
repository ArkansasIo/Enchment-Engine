use crate::Embedded;
use crate::prelude::*;
#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
use crate::self_update::{SelfUpdateEvent, SelfUpdater};
use crate::undo::character_undo::CharacterUndoAtom;
use crate::undo::item_undo::ItemUndoAtom;
use codegridfx::Module;
use rusterix::{
    PlayerCamera, Rusterix, SceneManager, SceneManagerResult, Texture, Value, ValueContainer,
};
use shared::rusterix_utils::*;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::mpsc::Receiver;
#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
use std::sync::{
    Arc, Mutex,
    mpsc::{Sender, channel},
};

#[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
use std::thread;

pub static PREVIEW_ICON: LazyLock<RwLock<(TheRGBATile, i32)>> =
    LazyLock::new(|| RwLock::new((TheRGBATile::default(), 0)));

pub static TILEPICKER: LazyLock<RwLock<TilePicker>> =
    LazyLock::new(|| RwLock::new(TilePicker::new("Main Tile Picker".to_string())));
pub static SHAPEPICKER: LazyLock<RwLock<ShapePicker>> =
    LazyLock::new(|| RwLock::new(ShapePicker::new("Main Shape Picker".to_string())));
pub static TILEMAPEDITOR: LazyLock<RwLock<TilemapEditor>> =
    LazyLock::new(|| RwLock::new(TilemapEditor::new()));
pub static SIDEBARMODE: LazyLock<RwLock<SidebarMode>> =
    LazyLock::new(|| RwLock::new(SidebarMode::Region));
pub static UNDOMANAGER: LazyLock<RwLock<UndoManager>> =
    LazyLock::new(|| RwLock::new(UndoManager::default()));
pub static TOOLLIST: LazyLock<RwLock<ToolList>> =
    LazyLock::new(|| RwLock::new(ToolList::default()));
pub static ACTIONLIST: LazyLock<RwLock<ActionList>> =
    LazyLock::new(|| RwLock::new(ActionList::default()));
// pub static PANELS: LazyLock<RwLock<Panels>> = LazyLock::new(|| RwLock::new(Panels::new()));
pub static CODEEDITOR: LazyLock<RwLock<CodeEditor>> =
    LazyLock::new(|| RwLock::new(CodeEditor::new()));
pub static PALETTE: LazyLock<RwLock<ThePalette>> =
    LazyLock::new(|| RwLock::new(ThePalette::default()));
pub static RUSTERIX: LazyLock<RwLock<Rusterix>> =
    LazyLock::new(|| RwLock::new(Rusterix::default()));
pub static CONFIGEDITOR: LazyLock<RwLock<ConfigEditor>> =
    LazyLock::new(|| RwLock::new(ConfigEditor::new()));
pub static INFOVIEWER: LazyLock<RwLock<InfoViewer>> =
    LazyLock::new(|| RwLock::new(InfoViewer::new()));
pub static CONFIG: LazyLock<RwLock<toml::Table>> =
    LazyLock::new(|| RwLock::new(toml::Table::default()));
pub static NODEEDITOR: LazyLock<RwLock<NodeEditor>> =
    LazyLock::new(|| RwLock::new(NodeEditor::new()));
pub static WORLDEDITOR: LazyLock<RwLock<WorldEditor>> =
    LazyLock::new(|| RwLock::new(WorldEditor::new()));
pub static RENDEREDITOR: LazyLock<RwLock<RenderEditor>> =
    LazyLock::new(|| RwLock::new(RenderEditor::new()));
pub static EDITCAMERA: LazyLock<RwLock<EditCamera>> =
    LazyLock::new(|| RwLock::new(EditCamera::new()));
pub static SCENEMANAGER: LazyLock<RwLock<SceneManager>> =
    LazyLock::new(|| RwLock::new(SceneManager::default()));
pub static DOCKMANAGER: LazyLock<RwLock<DockManager>> =
    LazyLock::new(|| RwLock::new(DockManager::default()));

pub static CODEGRIDFX: LazyLock<RwLock<Module>> =
    LazyLock::new(|| RwLock::new(Module::as_type(codegridfx::ModuleType::CharacterTemplate)));
pub static SHADEGRIDFX: LazyLock<RwLock<Module>> =
    LazyLock::new(|| RwLock::new(Module::as_type(codegridfx::ModuleType::Shader)));
pub static SHADERBUFFER: LazyLock<RwLock<TheRGBABuffer>> =
    LazyLock::new(|| RwLock::new(TheRGBABuffer::new(TheDim::sized(200, 200))));

pub struct Editor {
    project: Project,
    project_path: Option<PathBuf>,

    sidebar: Sidebar,
    mapeditor: MapEditor,

    server_ctx: ServerContext,

    update_tracker: UpdateTracker,
    event_receiver: Option<Receiver<TheEvent>>,

    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    self_update_rx: Receiver<SelfUpdateEvent>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    self_update_tx: Sender<SelfUpdateEvent>,
    #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
    self_updater: Arc<Mutex<SelfUpdater>>,

    update_counter: usize,

    build_values: ValueContainer,

    // Title page and state
    show_title_page: bool,

    // Workspace options / theme
    show_left_toolbar: bool,
    show_right_toolbar: bool,
    option_snap_to_grid: bool,
    option_show_grid: bool,
    option_show_gizmos: bool,
    theme_preset: String,
    left_toolbar_active_tool: Option<String>,
    left_group_modes_expanded: bool,
    left_group_2d_expanded: bool,
    left_group_3d_expanded: bool,
    left_group_editor_expanded: bool,
    towngen_preset: String,
    towngen_has_river: bool,
    towngen_has_walls: bool,
    towngen_last_seed: u64,
    towngen_auto_bake: bool,
    overlay_show_town_districts: bool,
    overlay_show_town_roads: bool,
    overlay_show_town_landmarks: bool,
    mmorpg_xp_rate: f32,
    mmorpg_loot_rate: f32,
    mmorpg_event_rate: f32,
    mmorpg_world_name: String,
    mmorpg_max_players: i32,
    mmorpg_starting_level: i32,
    mmorpg_race_count: i32,
    mmorpg_quest_count: i32,
    mmorpg_skill_tier_count: i32,
    mmorpg_include_warrior: bool,
    mmorpg_include_ranger: bool,
    mmorpg_include_mage: bool,
    mmorpg_include_cleric: bool,
    mmorpg_include_rogue: bool,
    fantasy_world_name: String,
    fantasy_world_seed: u64,
    fantasy_continent_count: i32,
    fantasy_countries_per_continent: i32,
    fantasy_towns_per_country: i32,
    fantasy_has_islands: bool,
    last_generated_town: Option<crate::game_logic::TownMapData>,
    last_generated_mmorpg: Option<crate::game_logic::StarterRpgMmorpgConfig>,
    last_generated_fantasy_world: Option<crate::game_logic::FantasyWorldMapData>,
}


impl Editor {
    fn is_compact_ui(&self, ctx: &TheContext) -> bool {
        ctx.width <= 1440 || ctx.height <= 860
    }

    fn is_tiny_ui(&self, ctx: &TheContext) -> bool {
        ctx.width <= 1280 || ctx.height <= 760
    }

    fn left_panel_width(&self, ctx: &TheContext) -> i32 {
        if self.is_tiny_ui(ctx) {
            ((ctx.width as i32 * 22) / 100).clamp(170, 300)
        } else if self.is_compact_ui(ctx) {
            ((ctx.width as i32 * 23) / 100).clamp(180, 330)
        } else {
            ((ctx.width as i32 * 24) / 100).clamp(190, 360)
        }
    }

    fn right_icon_panel_width(&self, ctx: &TheContext) -> i32 {
        if self.is_tiny_ui(ctx) {
            ((ctx.width as i32 * 8) / 100).clamp(72, 104)
        } else if self.is_compact_ui(ctx) {
            ((ctx.width as i32 * 9) / 100).clamp(80, 116)
        } else {
            ((ctx.width as i32 * 10) / 100).clamp(88, 128)
        }
    }

    fn right_settings_panel_width(&self, ctx: &TheContext) -> i32 {
        if self.is_tiny_ui(ctx) {
            ((ctx.width as i32 * 17) / 100).clamp(170, 250)
        } else if self.is_compact_ui(ctx) {
            ((ctx.width as i32 * 19) / 100).clamp(190, 280)
        } else {
            ((ctx.width as i32 * 20) / 100).clamp(200, 300)
        }
    }

    fn effective_side_widths(&self, ctx: &TheContext) -> (i32, i32) {
        let mut left = if self.show_left_toolbar {
            self.left_panel_width(ctx)
        } else {
            0
        };
        let mut right = if self.show_right_toolbar {
            self.right_settings_panel_width(ctx)
        } else {
            0
        };

        let min_center = if self.is_tiny_ui(ctx) {
            460
        } else if self.is_compact_ui(ctx) {
            560
        } else {
            700
        };
        let budget = (ctx.width as i32 - min_center).max(120);
        let total = left + right;
        if total > budget && total > 0 {
            left = (left * budget) / total;
            right = budget - left;
        }

        if self.show_left_toolbar {
            left = left.clamp(120, self.left_panel_width(ctx));
        }
        if self.show_right_toolbar {
            right = right.clamp(96, self.right_settings_panel_width(ctx));
        }
        (left, right)
    }

    fn is_realtime_mode(&self) -> bool {
        self.server_ctx.game_mode
            || RUSTERIX.read().unwrap().server.state == rusterix::ServerState::Running
    }

    fn redraw_interval_ms(&self) -> u64 {
        let config = CONFIGEDITOR.read().unwrap();
        if self.is_realtime_mode() {
            (1000 / config.target_fps.clamp(1, 60)) as u64
        } else {
            config.game_tick_ms.max(1) as u64
        }
    }

    fn apply_toolbar_visibility(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let (left_width, right_width) = self.effective_side_widths(ctx);

        if let Some(left_names_layout) = ui.get_layout("Left Tool Name Layout") {
            left_names_layout
                .limiter_mut()
                .set_max_width(if self.show_left_toolbar {
                    left_width
                } else {
                    0
                });
            left_names_layout.relayout(ctx);
        }
        if let Some(left_names_text_layout) = ui.get_text_layout("Left Tool Name Layout") {
            left_names_text_layout.set_fixed_text_width(8);
            left_names_text_layout.relayout(ctx);
        }
        if let Some(right_layout) = ui.get_layout("Right Tool Layout") {
            right_layout
                .limiter_mut()
                .set_max_width(if self.show_right_toolbar {
                    right_width.min(self.right_icon_panel_width(ctx))
                } else {
                    0
                });
            right_layout.relayout(ctx);
        }
        if let Some(right_settings_layout) = ui.get_text_layout("Right Settings Layout") {
            right_settings_layout
                .limiter_mut()
                .set_max_width(if self.show_right_toolbar { right_width } else { 0 });
            right_settings_layout.set_fixed_text_width((right_width - 108).max(92));
            right_settings_layout.relayout(ctx);
        }
        if let Some(widget) = ui.get_widget("Server Time Slider") {
            widget
                .limiter_mut()
                .set_max_width(((ctx.width as i32 * 22) / 100).clamp(120, 320));
        }
    }

    fn apply_theme_preset(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let top_color = match self.theme_preset.as_str() {
            "Light" => Some(TheThemeColors::TextLayoutBackground),
            "Slate" => Some(TheThemeColors::DefaultWidgetDarkBackground),
            _ => None,
        };
        let left_panel_color = match self.theme_preset.as_str() {
            "Light" => Some(TheThemeColors::TextLayoutBackground),
            "Slate" => Some(TheThemeColors::DefaultWidgetDarkBackground),
            _ => Some(TheThemeColors::DefaultWidgetDarkBackground),
        };
        let right_panel_color = match self.theme_preset.as_str() {
            "Light" => Some(TheThemeColors::TextLayoutBackground),
            "Slate" => Some(TheThemeColors::DefaultWidgetDarkBackground),
            _ => Some(TheThemeColors::ListLayoutBackground),
        };

        if let Some(layout) = ui.get_hlayout("Menu Layout") {
            layout.set_background_color(top_color);
            layout.relayout(ctx);
        }
        if let Some(layout) = ui.get_hlayout("Top Quick Tool Layout") {
            layout.set_background_color(top_color);
            layout.relayout(ctx);
        }
        if let Some(layout) = ui.get_layout("Left Tool Name Layout") {
            layout.set_background_color(left_panel_color);
            layout.relayout(ctx);
        }
        if let Some(layout) = ui.get_layout("Right Tool Layout") {
            layout.set_background_color(right_panel_color);
            layout.relayout(ctx);
        }
    }

    fn show_help_dialog(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let width = 760;
        let height = 430;

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(width, height));

        let mut layout = TheTextLayout::new(TheId::named("Help Dialog Layout"));
        layout.set_margin(Vec4::new(14, 14, 14, 14));
        layout.set_padding(8);
        layout.limiter_mut().set_max_width(width - 30);

        let mut intro = TheText::new(TheId::named("Help Intro"));
        intro.set_text("Enchentment Engine Help\nVersion: 0.8.100\nDeveloper: Markus Moenig\nWebsite: https://eldiron.com".to_string());
        layout.add_pair("".to_string(), Box::new(intro));

        let mut usage = TheText::new(TheId::named("Help Usage"));
        usage.set_text(
            "Workflow:\n\
             1) File -> New/Open project\n\
             2) Pick a tool from left/right bars or Tools menu\n\
             3) Edit map in viewport (2D/3D)\n\
             4) Build -> Play/Pause/Stop to test runtime\n\
             5) File -> Save / Export".to_string(),
        );
        layout.add_pair("".to_string(), Box::new(usage));

        let mut shortcuts = TheText::new(TheId::named("Help Shortcuts"));
        shortcuts.set_text(
            "Shortcuts:\n\
             S Selection, V Vertex, L Linedef, E Sector, R Rect, Y Entity,\n\
             W Terrain, N Render, C Code, D Data.\n\
             Arrow keys pan, mouse wheel zoom, Esc clears active selection/paste."
                .to_string(),
        );
        layout.add_pair("".to_string(), Box::new(shortcuts));

        let mut options = TheText::new(TheId::named("Help Options"));
        options.set_text(
            "Options & Theme:\n\
             Use right settings panel or View/Settings menus to change theme,\n\
             snap-to-grid, grid visibility, gizmos, fps, tick rate, and toolbar layout."
                .to_string(),
        );
        layout.add_pair("".to_string(), Box::new(options));

        let mut town_help = TheText::new(TheId::named("Help TownSystems"));
        town_help.set_text(
            "Town + MMO Systems:\n\
             Tools -> Town Size Presets / Reseed / Regenerate / Bake To Current Map.\n\
             Right panel includes Town controls, overlays, export/import JSON,\n\
             quest autogen, POI spawn, and MMO simulation buttons (tick/combat/loot)."
                .to_string(),
        );
        layout.add_pair("".to_string(), Box::new(town_help));

        canvas.set_layout(layout);
        ui.show_dialog("Help", canvas, vec![TheDialogButtonRole::Accept], ctx);
    }

    fn show_about_dialog(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let width = 620;
        let height = 280;

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(width, height));

        let mut layout = TheTextLayout::new(TheId::named("About Dialog Layout"));
        layout.set_margin(Vec4::new(14, 14, 14, 14));
        layout.set_padding(8);
        layout.limiter_mut().set_max_width(width - 30);

        let mut title = TheText::new(TheId::named("About Title"));
        title.set_text("Enchentment Engine Creator v0.8.100".to_string());
        layout.add_pair("".to_string(), Box::new(title));

        let mut body = TheText::new(TheId::named("About Body"));
        body.set_text(
            "A modular retro-style 2D/3D RPG/MMORPG engine and editor.\n\
             Developed by Markus Moenig.\n\
             Version: 0.8.100\n\
             Website: https://eldiron.com\n\
             Source: https://github.com/markusmoenig/Eldiron\n\
             \nFeatures:\n\
             - 2D/3D editing workflows\n\
             - Unreal-style IDE layout\n\
             - Town & world generation\n\
             - RPG/MMORPG simulation\n\
             - Scripting, content pipelines, export stubs\n\
             - In-editor help and about dialogs"
            .to_string(),
        );
        layout.add_pair("".to_string(), Box::new(body));

        canvas.set_layout(layout);
        ui.show_dialog("About Encheament Engine", canvas, vec![TheDialogButtonRole::Accept], ctx);
    }

    fn apply_ide_layout_unreal(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        self.show_left_toolbar = true;
        self.show_right_toolbar = true;
        self.option_snap_to_grid = true;
        self.option_show_grid = true;
        self.option_show_gizmos = true;
        self.theme_preset = "Slate".to_string();
        self.left_group_modes_expanded = true;
        self.left_group_2d_expanded = true;
        self.left_group_3d_expanded = true;
        self.left_group_editor_expanded = true;

        self.apply_toolbar_visibility(ui, ctx);
        self.apply_theme_preset(ui, ctx);
        self.rebuild_left_tool_name_layout(ui, ctx);
        self.apply_workspace_settings_to_ui(ui, ctx);
        self.persist_workspace_settings_to_project_config();
    }

    fn apply_ide_layout_minimal(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        self.show_left_toolbar = true;
        self.show_right_toolbar = false;
        self.option_snap_to_grid = false;
        self.option_show_grid = false;
        self.option_show_gizmos = false;
        self.theme_preset = "Dark".to_string();
        self.left_group_modes_expanded = true;
        self.left_group_2d_expanded = false;
        self.left_group_3d_expanded = false;
        self.left_group_editor_expanded = true;

        self.apply_toolbar_visibility(ui, ctx);
        self.apply_theme_preset(ui, ctx);
        self.rebuild_left_tool_name_layout(ui, ctx);
        self.apply_workspace_settings_to_ui(ui, ctx);
        self.persist_workspace_settings_to_project_config();
    }

    fn show_ide_feature_matrix_dialog(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let width = 880;
        let height = 520;

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(width, height));

        let mut layout = TheTextLayout::new(TheId::named("IDE Feature Matrix Layout"));
        layout.set_margin(Vec4::new(14, 14, 14, 14));
        layout.set_padding(8);
        layout.limiter_mut().set_max_width(width - 30);

        let profile = crate::ide_profile::IdeProfile::unreal_like();

        let mut title = TheText::new(TheId::named("IDE Feature Matrix Title"));
        title.set_text(format!("IDE Profile: {}", profile.name));
        layout.add_pair("".to_string(), Box::new(title));

        for category in profile.categories {
            let mut cat = TheText::new(TheId::named(&format!("IDE Category {}", category.name)));
            cat.set_text(format!("{}:", category.name));
            layout.add_pair("".to_string(), Box::new(cat));

            for feature in category.features {
                let mut line =
                    TheText::new(TheId::named(&format!("IDE Feature {}", feature.name)));
                line.set_text(format!(" - {}: {}", feature.name, feature.description));
                layout.add_pair("".to_string(), Box::new(line));
            }
        }

        canvas.set_layout(layout);
        ui.show_dialog(
            "IDE Feature Matrix",
            canvas,
            vec![TheDialogButtonRole::Accept],
            ctx,
        );
    }

    fn show_ide_panel_dialog(
        &self,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        kind: crate::ide_panels::IdePanelKind,
    ) {
        let snapshot = crate::ide_panels::IdePanelSnapshot::from_editor_state(
            &self.project,
            &self.server_ctx,
            self.last_generated_town.is_some(),
            self.last_generated_mmorpg.is_some(),
        );
        crate::ide_panels::show_ide_panel_dialog(ui, ctx, kind, &snapshot);
    }

    fn show_rpg_mmorpg_builder_dialog(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(700, 360));

        let mut layout = TheTextLayout::new(TheId::named("RpgMmorpg Builder Dialog"));
        layout.set_margin(Vec4::new(12, 12, 12, 12));
        layout.set_padding(6);

        let mut txt = TheText::new(TheId::named("RpgMmorpg Builder Dialog Text"));
        txt.set_text(format!(
            "RPG/MMORPG Builder Inputs\n\
             World: {}\n\
             Max Players: {}\n\
             Starting Level: {}\n\
             Race Types: {}\n\
             Quest Count: {}\n\
             Skill Tiers: {}\n\
             Classes Enabled: Warrior={} Ranger={} Mage={} Cleric={} Rogue={}\n\
             Rates: XP={} Loot={} Event={}\n\
             \nUse right panel fields or Tools -> RPG/MMORPG Builder -> Generate from Builder.",
            self.mmorpg_world_name,
            self.mmorpg_max_players,
            self.mmorpg_starting_level,
            self.mmorpg_race_count,
            self.mmorpg_quest_count,
            self.mmorpg_skill_tier_count,
            self.mmorpg_include_warrior,
            self.mmorpg_include_ranger,
            self.mmorpg_include_mage,
            self.mmorpg_include_cleric,
            self.mmorpg_include_rogue,
            self.mmorpg_xp_rate,
            self.mmorpg_loot_rate,
            self.mmorpg_event_rate
        ));
        layout.add_pair("".to_string(), Box::new(txt));

        canvas.set_layout(layout);
        ui.show_dialog(
            "RPG/MMORPG Builder",
            canvas,
            vec![TheDialogButtonRole::Accept],
            ctx,
        );
    }

    fn tool_group_label(&self, tool_name: &str, fallback: &str) -> String {
        match tool_name {
            "Select Tool" => "Modes / Select".to_string(),
            "Vertex Tool" => "2D / Vertex".to_string(),
            "Linedef Tool" => "2D / Linedef".to_string(),
            "Sector Tool" => "2D / Sector".to_string(),
            "Rect Tool" => "2D / Rect".to_string(),
            "Entity Tool" => "3D / Entity".to_string(),
            "Render Tool" => "3D / Render".to_string(),
            "World Tool" => "3D / Terrain".to_string(),
            "Code Tool" => "Editor / Code".to_string(),
            "Data Tool" => "Editor / Data".to_string(),
            "Tileset Tool" => "Editor / Tileset".to_string(),
            "Config Tool" => "Editor / Config".to_string(),
            "Info Tool" => "Editor / Info".to_string(),
            "Game Tool" => "Play / Runtime".to_string(),
            _ => fallback.to_string(),
        }
    }

    fn left_tool_group_key(&self, tool_name: &str) -> &'static str {
        match tool_name {
            "Select Tool" | "Rect Tool" => "Modes",
            "Vertex Tool" | "Linedef Tool" | "Sector Tool" => "2D",
            "Entity Tool" | "Render Tool" | "World Tool" | "Game Tool" => "3D",
            _ => "Editor",
        }
    }

    fn build_left_ue5_tool_panel(&self, _ctx: &mut TheContext) -> TheCanvas {
        let mut tool_list_canvas: TheCanvas = TheCanvas::new();
        let mut tool_list_bar_canvas = TheCanvas::new();
        tool_list_bar_canvas.set_widget(TheToolListBar::new(TheId::named("Left Tools Bar")));
        tool_list_canvas.set_top(tool_list_bar_canvas);
        let (left_width, _) = self.effective_side_widths(_ctx);

        let mut left_tool_name_layout = TheTextLayout::new(TheId::named("Left Tool Name Layout"));
        left_tool_name_layout
            .limiter_mut()
            .set_max_width(if self.show_left_toolbar {
                left_width
            } else {
                0
            });
        if self.is_tiny_ui(_ctx) {
            left_tool_name_layout.set_margin(Vec4::new(2, 2, 2, 2));
            left_tool_name_layout.set_padding(1);
        } else {
            left_tool_name_layout.set_margin(Vec4::new(4, 4, 4, 4));
            left_tool_name_layout.set_padding(3);
        }
        left_tool_name_layout.set_fixed_text_width(8);

        self.populate_left_tool_name_layout(&mut left_tool_name_layout);
        tool_list_canvas.set_layout(left_tool_name_layout);
        tool_list_canvas
    }

    fn build_right_ue5_panel(&self, ctx: &TheContext) -> TheCanvas {
        let mut right_tool_canvas = TheCanvas::new();
        let compact = self.is_compact_ui(ctx);
        let tiny = self.is_tiny_ui(ctx);
        let (_, right_width) = self.effective_side_widths(ctx);

        let mut right_tool_bar_canvas = TheCanvas::new();
        right_tool_bar_canvas.set_widget(TheToolListBar::new(TheId::named("Right Tool Bar")));
        right_tool_canvas.set_top(right_tool_bar_canvas);

        let mut right_tool_layout = TheVLayout::new(TheId::named("Right Tool Layout"));
        right_tool_layout
            .limiter_mut()
            .set_max_width(right_width.min(self.right_icon_panel_width(ctx)));
        if tiny {
            right_tool_layout.set_margin(Vec4::new(1, 1, 1, 1));
            right_tool_layout.set_padding(0);
        } else {
            right_tool_layout.set_margin(Vec4::new(2, 2, 2, 2));
            right_tool_layout.set_padding(1);
        }

        let mut add_quick_button = |id: &str, icon: &str, label: &str, status: &str| {
            let mut button = TheTraybarButton::new(TheId::named(id));
            button.set_icon_name(icon.to_string());
            if compact {
                button.set_text(String::new());
            } else {
                button.set_text(label.to_string());
            }
            button.set_status_text(status);
            right_tool_layout.add_widget(Box::new(button));
        };

        add_quick_button("QuickGuiOpen", "icon_role_load", "Open", "Open project");
        add_quick_button("QuickGuiSave", "icon_role_save", "Save", "Save project");
        add_quick_button("QuickGuiUndo", "icon_role_undo", "Undo", "Undo");
        add_quick_button("QuickGuiRedo", "icon_role_redo", "Redo", "Redo");
        add_quick_button("QuickGuiPlay", "play", "Play", "Run game");
        add_quick_button("QuickGuiPause", "play-pause", "Pause", "Pause game");
        add_quick_button("QuickGuiStop", "stop-fill", "Stop", "Stop game");
        add_quick_button("QuickGuiHelp", "question-mark", "Help", "Open help");
        add_quick_button(
            "QuickWindowContentBrowser",
            "folder",
            "Content",
            "Open Content Browser panel",
        );
        add_quick_button(
            "QuickWindowOutliner",
            "list",
            "Outliner",
            "Open World Outliner panel",
        );
        add_quick_button("QuickWindowDetails", "gear", "Details", "Open Details panel");
        add_quick_button("QuickWindowLog", "file-text", "Log", "Open Output Log panel");
        add_quick_button("QuickWindowBlueprint", "code", "Blueprint", "Open Blueprint panel");
        add_quick_button(
            "QuickMmorpgBuilder",
            "list",
            "MMO UI",
            "Open RPG/MMORPG builder summary",
        );
        add_quick_button(
            "QuickMmorpgGenerate",
            "wand-magic-sparkles",
            "MMO Gen",
            "Generate RPG/MMORPG from builder inputs",
        );
        add_quick_button("QuickThemeDark", "dark_tabbar_selected", "Dark", "Dark theme preset");
        add_quick_button("QuickThemeLight", "dark_tabbar_hover", "Light", "Light theme preset");
        add_quick_button("QuickThemeSlate", "dark_tabbar_normal", "Slate", "Slate theme preset");
        add_quick_button("QuickOptSnap", "selection", "Snap", "Toggle snap-to-grid");
        add_quick_button("QuickOptGrid", "square", "Grid", "Toggle grid visibility");
        add_quick_button("QuickOptGizmos", "transform", "Gizmos", "Toggle gizmos");

        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let quick_id = format!("RightTool::{}", tool.id().name);
                let mut button = TheMenubarButton::new(TheId::named(&quick_id));
                button.set_icon_name(tool.icon_name());
                button.set_has_state(true);
                button.set_status_text(&format!("Activate {}", tool.info()));
                right_tool_layout.add_widget(Box::new(button));
            }
        }
        right_tool_canvas.set_layout(right_tool_layout);

        let mut right_settings_canvas = TheCanvas::new();
        right_settings_canvas
            .set_widget(TheTraybar::new(TheId::named("Right Settings Bar")));

        let mut right_settings_layout = TheTextLayout::new(TheId::named("Right Settings Layout"));
        if tiny {
            right_settings_layout.set_margin(Vec4::new(2, 2, 2, 2));
            right_settings_layout.set_padding(2);
        } else {
            right_settings_layout.set_margin(Vec4::new(4, 4, 4, 4));
            right_settings_layout.set_padding(4);
        }
        right_settings_layout
            .limiter_mut()
            .set_max_width(right_width);
        right_settings_layout.set_fixed_text_width((right_width - 108).max(92));

        let mut theme_dropdown = TheDropdownMenu::new(TheId::named("ThemePresetDropdown"));
        theme_dropdown.add_option("Dark".to_string());
        theme_dropdown.add_option("Light".to_string());
        theme_dropdown.add_option("Slate".to_string());
        let theme_index = match self.theme_preset.as_str() {
            "Light" => 1,
            "Slate" => 2,
            _ => 0,
        };
        theme_dropdown.set_selected_index(theme_index);
        right_settings_layout.add_pair("Details / Theme".to_string(), Box::new(theme_dropdown));

        let mut snap_cb = TheCheckButton::new(TheId::named("OptionSnapCB"));
        snap_cb.set_value(TheValue::Bool(self.option_snap_to_grid));
        right_settings_layout.add_pair("Details / Snap".to_string(), Box::new(snap_cb));

        let mut grid_cb = TheCheckButton::new(TheId::named("OptionGridCB"));
        grid_cb.set_value(TheValue::Bool(self.option_show_grid));
        right_settings_layout.add_pair("Viewport / Grid".to_string(), Box::new(grid_cb));

        let mut gizmo_cb = TheCheckButton::new(TheId::named("OptionGizmoCB"));
        gizmo_cb.set_value(TheValue::Bool(self.option_show_gizmos));
        right_settings_layout.add_pair("Viewport / Gizmos".to_string(), Box::new(gizmo_cb));

        let mut left_cb = TheCheckButton::new(TheId::named("OptionLeftToolbarCB"));
        left_cb.set_value(TheValue::Bool(self.show_left_toolbar));
        right_settings_layout.add_pair("Layout / Left Panel".to_string(), Box::new(left_cb));

        let mut right_cb = TheCheckButton::new(TheId::named("OptionRightToolbarCB"));
        right_cb.set_value(TheValue::Bool(self.show_right_toolbar));
        right_settings_layout.add_pair("Layout / Right Panel".to_string(), Box::new(right_cb));

        let mut fps_edit = TheTextLineEdit::new(TheId::named("OptionTargetFpsEdit"));
        fps_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().target_fps));
        fps_edit.set_range(TheValue::RangeI32(1..=120));
        fps_edit.set_continuous(true);
        right_settings_layout.add_pair("Runtime / FPS".to_string(), Box::new(fps_edit));

        let mut tick_edit = TheTextLineEdit::new(TheId::named("OptionTickMsEdit"));
        tick_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().game_tick_ms));
        tick_edit.set_range(TheValue::RangeI32(10..=2000));
        tick_edit.set_continuous(true);
        right_settings_layout.add_pair("Runtime / Tick ms".to_string(), Box::new(tick_edit));

        let mut grid_size_edit = TheTextLineEdit::new(TheId::named("OptionGridSizeEdit"));
        grid_size_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().grid_size));
        grid_size_edit.set_range(TheValue::RangeI32(4..=256));
        grid_size_edit.set_continuous(true);
        right_settings_layout.add_pair("Viewport / Grid Size".to_string(), Box::new(grid_size_edit));

        let mut town_preset_dropdown = TheDropdownMenu::new(TheId::named("TownPresetDropdown"));
        town_preset_dropdown.add_option("Small Town".to_string());
        town_preset_dropdown.add_option("Large Town".to_string());
        town_preset_dropdown.add_option("Small City".to_string());
        town_preset_dropdown.add_option("Large City".to_string());
        let town_preset_index = match self.towngen_preset.as_str() {
            "Small Town" => 0,
            "Large Town" => 1,
            "Large City" => 3,
            _ => 2,
        };
        town_preset_dropdown.set_selected_index(town_preset_index);
        right_settings_layout.add_pair("Town / Preset".to_string(), Box::new(town_preset_dropdown));

        let mut town_seed_edit = TheTextLineEdit::new(TheId::named("TownSeedEdit"));
        town_seed_edit.set_value(TheValue::Int(self.towngen_last_seed as i32));
        town_seed_edit.set_range(TheValue::RangeI32(1..=i32::MAX));
        town_seed_edit.set_continuous(true);
        right_settings_layout.add_pair("Town / Seed".to_string(), Box::new(town_seed_edit));

        let mut town_river_cb = TheCheckButton::new(TheId::named("TownRiverCB"));
        town_river_cb.set_value(TheValue::Bool(self.towngen_has_river));
        right_settings_layout.add_pair("Town / River".to_string(), Box::new(town_river_cb));

        let mut town_walls_cb = TheCheckButton::new(TheId::named("TownWallsCB"));
        town_walls_cb.set_value(TheValue::Bool(self.towngen_has_walls));
        right_settings_layout.add_pair("Town / Walls".to_string(), Box::new(town_walls_cb));

        let mut town_auto_bake_cb = TheCheckButton::new(TheId::named("TownAutoBakeCB"));
        town_auto_bake_cb.set_value(TheValue::Bool(self.towngen_auto_bake));
        right_settings_layout.add_pair("Town / Auto Bake".to_string(), Box::new(town_auto_bake_cb));

        let mut show_districts_cb = TheCheckButton::new(TheId::named("OverlayTownDistrictsCB"));
        show_districts_cb.set_value(TheValue::Bool(self.overlay_show_town_districts));
        right_settings_layout.add_pair("Overlay / Districts".to_string(), Box::new(show_districts_cb));

        let mut show_roads_cb = TheCheckButton::new(TheId::named("OverlayTownRoadsCB"));
        show_roads_cb.set_value(TheValue::Bool(self.overlay_show_town_roads));
        right_settings_layout.add_pair("Overlay / Roads".to_string(), Box::new(show_roads_cb));

        let mut show_landmarks_cb = TheCheckButton::new(TheId::named("OverlayTownLandmarksCB"));
        show_landmarks_cb.set_value(TheValue::Bool(self.overlay_show_town_landmarks));
        right_settings_layout.add_pair("Overlay / Landmarks".to_string(), Box::new(show_landmarks_cb));

        let mut town_generate_btn = TheTraybarButton::new(TheId::named("TownGenerateBtn"));
        town_generate_btn.set_text("Generate".to_string());
        right_settings_layout.add_pair("Town / Action".to_string(), Box::new(town_generate_btn));

        let mut town_regen_btn = TheTraybarButton::new(TheId::named("TownRegenerateBtn"));
        town_regen_btn.set_text("Regenerate".to_string());
        right_settings_layout.add_pair("Town / Action".to_string(), Box::new(town_regen_btn));

        let mut town_bake_btn = TheTraybarButton::new(TheId::named("TownBakeBtn"));
        town_bake_btn.set_text("Bake To Map".to_string());
        right_settings_layout.add_pair("Town / Action".to_string(), Box::new(town_bake_btn));

        let mut export_btn = TheTraybarButton::new(TheId::named("TownExportBtn"));
        export_btn.set_text("Export Town JSON".to_string());
        right_settings_layout.add_pair("Town / Data".to_string(), Box::new(export_btn));

        let mut import_btn = TheTraybarButton::new(TheId::named("TownImportBtn"));
        import_btn.set_text("Import Town JSON".to_string());
        right_settings_layout.add_pair("Town / Data".to_string(), Box::new(import_btn));

        let mut quest_btn = TheTraybarButton::new(TheId::named("TownAutoQuestBtn"));
        quest_btn.set_text("Generate Quests".to_string());
        right_settings_layout.add_pair("Town / MMO".to_string(), Box::new(quest_btn));

        let mut spawner_btn = TheTraybarButton::new(TheId::named("TownSpawnPoiBtn"));
        spawner_btn.set_text("Spawn POIs".to_string());
        right_settings_layout.add_pair("Town / MMO".to_string(), Box::new(spawner_btn));

        let mut fantasy_world_name = TheTextLineEdit::new(TheId::named("FantasyWorldNameEdit"));
        fantasy_world_name.set_value(TheValue::Text(self.fantasy_world_name.clone()));
        fantasy_world_name.set_continuous(true);
        right_settings_layout.add_pair("Fantasy / World Name".to_string(), Box::new(fantasy_world_name));

        let mut fantasy_world_seed = TheTextLineEdit::new(TheId::named("FantasyWorldSeedEdit"));
        fantasy_world_seed.set_value(TheValue::Int(self.fantasy_world_seed as i32));
        fantasy_world_seed.set_range(TheValue::RangeI32(0..=i32::MAX));
        fantasy_world_seed.set_continuous(true);
        right_settings_layout.add_pair("Fantasy / Seed".to_string(), Box::new(fantasy_world_seed));

        let mut fantasy_continents = TheTextLineEdit::new(TheId::named("FantasyContinentCountEdit"));
        fantasy_continents.set_value(TheValue::Int(self.fantasy_continent_count));
        fantasy_continents.set_range(TheValue::RangeI32(1..=12));
        fantasy_continents.set_continuous(true);
        right_settings_layout.add_pair("Fantasy / Continents".to_string(), Box::new(fantasy_continents));

        let mut fantasy_countries =
            TheTextLineEdit::new(TheId::named("FantasyCountriesPerContinentEdit"));
        fantasy_countries.set_value(TheValue::Int(self.fantasy_countries_per_continent));
        fantasy_countries.set_range(TheValue::RangeI32(1..=24));
        fantasy_countries.set_continuous(true);
        right_settings_layout.add_pair(
            "Fantasy / Countries/Continent".to_string(),
            Box::new(fantasy_countries),
        );

        let mut fantasy_towns = TheTextLineEdit::new(TheId::named("FantasyTownsPerCountryEdit"));
        fantasy_towns.set_value(TheValue::Int(self.fantasy_towns_per_country));
        fantasy_towns.set_range(TheValue::RangeI32(1..=20));
        fantasy_towns.set_continuous(true);
        right_settings_layout.add_pair("Fantasy / Towns/Country".to_string(), Box::new(fantasy_towns));

        let mut fantasy_islands = TheCheckButton::new(TheId::named("FantasyHasIslandsCB"));
        fantasy_islands.set_value(TheValue::Bool(self.fantasy_has_islands));
        right_settings_layout.add_pair("Fantasy / Islands".to_string(), Box::new(fantasy_islands));

        let mut fantasy_generate_btn =
            TheTraybarButton::new(TheId::named("FantasyWorldGenerateBtn"));
        fantasy_generate_btn.set_text("Generate Fantasy World".to_string());
        right_settings_layout.add_pair("Fantasy / Action".to_string(), Box::new(fantasy_generate_btn));

        let mut xp_rate_edit = TheTextLineEdit::new(TheId::named("MmorpgXpRateEdit"));
        xp_rate_edit.set_value(TheValue::Float(self.mmorpg_xp_rate));
        xp_rate_edit.set_range(TheValue::RangeF32(0.1..=5.0));
        xp_rate_edit.set_continuous(true);
        right_settings_layout.add_pair("MMO / XP Rate".to_string(), Box::new(xp_rate_edit));

        let mut loot_rate_edit = TheTextLineEdit::new(TheId::named("MmorpgLootRateEdit"));
        loot_rate_edit.set_value(TheValue::Float(self.mmorpg_loot_rate));
        loot_rate_edit.set_range(TheValue::RangeF32(0.1..=5.0));
        loot_rate_edit.set_continuous(true);
        right_settings_layout.add_pair("MMO / Loot Rate".to_string(), Box::new(loot_rate_edit));

        let mut event_rate_edit = TheTextLineEdit::new(TheId::named("MmorpgEventRateEdit"));
        event_rate_edit.set_value(TheValue::Float(self.mmorpg_event_rate));
        event_rate_edit.set_range(TheValue::RangeF32(0.1..=5.0));
        event_rate_edit.set_continuous(true);
        right_settings_layout.add_pair("MMO / Event Rate".to_string(), Box::new(event_rate_edit));

        let mut mmorpg_world_name = TheTextLineEdit::new(TheId::named("MmorpgWorldNameEdit"));
        mmorpg_world_name.set_value(TheValue::Text(self.mmorpg_world_name.clone()));
        mmorpg_world_name.set_continuous(true);
        right_settings_layout.add_pair("MMO / World Name".to_string(), Box::new(mmorpg_world_name));

        let mut mmorpg_max_players = TheTextLineEdit::new(TheId::named("MmorpgMaxPlayersEdit"));
        mmorpg_max_players.set_value(TheValue::Int(self.mmorpg_max_players));
        mmorpg_max_players.set_range(TheValue::RangeI32(10..=100_000));
        mmorpg_max_players.set_continuous(true);
        right_settings_layout.add_pair("MMO / Max Players".to_string(), Box::new(mmorpg_max_players));

        let mut mmorpg_starting_level =
            TheTextLineEdit::new(TheId::named("MmorpgStartingLevelEdit"));
        mmorpg_starting_level.set_value(TheValue::Int(self.mmorpg_starting_level));
        mmorpg_starting_level.set_range(TheValue::RangeI32(1..=99));
        mmorpg_starting_level.set_continuous(true);
        right_settings_layout.add_pair(
            "RPG / Starting Level".to_string(),
            Box::new(mmorpg_starting_level),
        );

        let mut mmorpg_race_count = TheTextLineEdit::new(TheId::named("MmorpgRaceCountEdit"));
        mmorpg_race_count.set_value(TheValue::Int(self.mmorpg_race_count));
        mmorpg_race_count.set_range(TheValue::RangeI32(1..=12));
        mmorpg_race_count.set_continuous(true);
        right_settings_layout.add_pair("RPG / Race Types".to_string(), Box::new(mmorpg_race_count));

        let mut mmorpg_quest_count = TheTextLineEdit::new(TheId::named("MmorpgQuestCountEdit"));
        mmorpg_quest_count.set_value(TheValue::Int(self.mmorpg_quest_count));
        mmorpg_quest_count.set_range(TheValue::RangeI32(1..=64));
        mmorpg_quest_count.set_continuous(true);
        right_settings_layout.add_pair("RPG / Quest Count".to_string(), Box::new(mmorpg_quest_count));

        let mut mmorpg_skill_tier_count =
            TheTextLineEdit::new(TheId::named("MmorpgSkillTierCountEdit"));
        mmorpg_skill_tier_count.set_value(TheValue::Int(self.mmorpg_skill_tier_count));
        mmorpg_skill_tier_count.set_range(TheValue::RangeI32(1..=6));
        mmorpg_skill_tier_count.set_continuous(true);
        right_settings_layout.add_pair(
            "RPG / Skill Tiers".to_string(),
            Box::new(mmorpg_skill_tier_count),
        );

        let mut class_warrior_cb = TheCheckButton::new(TheId::named("MmorpgClassWarriorCB"));
        class_warrior_cb.set_value(TheValue::Bool(self.mmorpg_include_warrior));
        right_settings_layout.add_pair("RPG / Class Warrior".to_string(), Box::new(class_warrior_cb));

        let mut class_ranger_cb = TheCheckButton::new(TheId::named("MmorpgClassRangerCB"));
        class_ranger_cb.set_value(TheValue::Bool(self.mmorpg_include_ranger));
        right_settings_layout.add_pair("RPG / Class Ranger".to_string(), Box::new(class_ranger_cb));

        let mut class_mage_cb = TheCheckButton::new(TheId::named("MmorpgClassMageCB"));
        class_mage_cb.set_value(TheValue::Bool(self.mmorpg_include_mage));
        right_settings_layout.add_pair("RPG / Class Mage".to_string(), Box::new(class_mage_cb));

        let mut class_cleric_cb = TheCheckButton::new(TheId::named("MmorpgClassClericCB"));
        class_cleric_cb.set_value(TheValue::Bool(self.mmorpg_include_cleric));
        right_settings_layout.add_pair("RPG / Class Cleric".to_string(), Box::new(class_cleric_cb));

        let mut class_rogue_cb = TheCheckButton::new(TheId::named("MmorpgClassRogueCB"));
        class_rogue_cb.set_value(TheValue::Bool(self.mmorpg_include_rogue));
        right_settings_layout.add_pair("RPG / Class Rogue".to_string(), Box::new(class_rogue_cb));

        let mut mmorpg_generate_btn = TheTraybarButton::new(TheId::named("MmorpgGenerateBtn"));
        mmorpg_generate_btn.set_text("Generate RPG/MMO".to_string());
        right_settings_layout.add_pair("MMO / Build".to_string(), Box::new(mmorpg_generate_btn));

        let mut sim_tick_btn = TheTraybarButton::new(TheId::named("MmorpgSimTickBtn"));
        sim_tick_btn.set_text("Sim Tick".to_string());
        right_settings_layout.add_pair("MMO / Sim".to_string(), Box::new(sim_tick_btn));

        let mut sim_combat_btn = TheTraybarButton::new(TheId::named("MmorpgSimCombatBtn"));
        sim_combat_btn.set_text("Sim Combat".to_string());
        right_settings_layout.add_pair("MMO / Sim".to_string(), Box::new(sim_combat_btn));

        let mut sim_loot_btn = TheTraybarButton::new(TheId::named("MmorpgSimLootBtn"));
        sim_loot_btn.set_text("Sim Loot".to_string());
        right_settings_layout.add_pair("MMO / Sim".to_string(), Box::new(sim_loot_btn));

        right_settings_canvas.set_layout(right_settings_layout);
        right_tool_canvas.set_bottom(right_settings_canvas);

        let mut right_tool_border_canvas = TheCanvas::new();
        let mut right_border_widget = TheIconView::new(TheId::empty());
        right_border_widget.set_border_color(Some([82, 82, 82, 255]));
        right_border_widget.limiter_mut().set_max_width(1);
        right_border_widget.limiter_mut().set_max_height(i32::MAX);
        right_tool_border_canvas.set_widget(right_border_widget);
        right_tool_canvas.set_left(right_tool_border_canvas);

        right_tool_canvas
    }

    fn build_ue5_workspace_canvas(&mut self, ui: &mut TheUI, ctx: &mut TheContext) -> TheCanvas {
        self.sidebar.init_ui(ui, ctx, &mut self.server_ctx);

        let bottom_panels = DOCKMANAGER.write().unwrap().init(ctx);

        let mut editor_canvas: TheCanvas = TheCanvas::new();
        let mut editor_stack = TheStackLayout::new(TheId::named("Editor Stack"));
        let poly_canvas = self.mapeditor.init_ui(ui, ctx, &mut self.project);
        editor_stack.add_canvas(poly_canvas);
        DOCKMANAGER
            .write()
            .unwrap()
            .add_editors_to_stack(&mut editor_stack, ctx);
        editor_canvas.set_layout(editor_stack);

        let mut viewport_utilities_layout =
            TheSharedVLayout::new(TheId::named("UE5 Viewport Utilities Layout"));
        viewport_utilities_layout.add_canvas(editor_canvas);
        viewport_utilities_layout.add_canvas(bottom_panels);
        viewport_utilities_layout.set_shared_ratio(crate::DEFAULT_VLAYOUT_RATIO);
        viewport_utilities_layout.set_mode(TheSharedVLayoutMode::Shared);

        let mut workspace_canvas = TheCanvas::new();
        workspace_canvas.set_layout(viewport_utilities_layout);
        workspace_canvas.set_left(self.build_left_ue5_tool_panel(ctx));
        workspace_canvas.set_right(self.build_right_ue5_panel(ctx));
        workspace_canvas
    }

    fn build_status_canvas(&self) -> TheCanvas {
        let mut status_canvas = TheCanvas::new();
        let mut statusbar = TheStatusbar::new(TheId::named("Statusbar"));
        statusbar.set_text(fl!("info_welcome"));
        status_canvas.set_widget(statusbar);
        status_canvas
    }

    fn set_left_toolbar_active_tool(&mut self, ctx: &mut TheContext, tool_name: &str) {
        if self.left_toolbar_active_tool.as_deref() == Some(tool_name) {
            return;
        }

        let group_changed = match self.left_tool_group_key(tool_name) {
            "Modes" => {
                let changed = !self.left_group_modes_expanded;
                self.left_group_modes_expanded = true;
                changed
            }
            "2D" => {
                let changed = !self.left_group_2d_expanded;
                self.left_group_2d_expanded = true;
                changed
            }
            "3D" => {
                let changed = !self.left_group_3d_expanded;
                self.left_group_3d_expanded = true;
                changed
            }
            _ => {
                let changed = !self.left_group_editor_expanded;
                self.left_group_editor_expanded = true;
                changed
            }
        };

        if group_changed {
            ctx.ui.send(TheEvent::Custom(
                TheId::named("Rebuild Left Toolbar"),
                TheValue::Empty,
            ));
            self.persist_workspace_settings_to_project_config();
        }

        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let left_tool_id = format!("LeftTool::{}", tool.id().name);
                let top_tool_id = format!("TopTool::{}", tool.id().name);
                let right_tool_id = format!("RightTool::{}", tool.id().name);
                ctx.ui.set_widget_state(left_tool_id, TheWidgetState::None);
                ctx.ui.set_widget_state(top_tool_id, TheWidgetState::None);
                ctx.ui.set_widget_state(right_tool_id, TheWidgetState::None);
            }
        }

        ctx.ui
            .set_widget_state("LeftMode::Select Tool".to_string(), TheWidgetState::None);
        ctx.ui
            .set_widget_state("LeftMode::World Tool".to_string(), TheWidgetState::None);
        ctx.ui
            .set_widget_state("LeftMode::Entity Tool".to_string(), TheWidgetState::None);
        ctx.ui
            .set_widget_state("LeftMode::Rect Tool".to_string(), TheWidgetState::None);

        ctx.ui.set_widget_state(
            format!("LeftTool::{}", tool_name),
            TheWidgetState::Selected,
        );
        ctx.ui
            .set_widget_state(format!("TopTool::{}", tool_name), TheWidgetState::Selected);
        ctx.ui.set_widget_state(
            format!("RightTool::{}", tool_name),
            TheWidgetState::Selected,
        );

        match tool_name {
            "Select Tool" => ctx.ui.set_widget_state(
                "LeftMode::Select Tool".to_string(),
                TheWidgetState::Selected,
            ),
            "World Tool" => ctx.ui.set_widget_state(
                "LeftMode::World Tool".to_string(),
                TheWidgetState::Selected,
            ),
            "Entity Tool" => ctx.ui.set_widget_state(
                "LeftMode::Entity Tool".to_string(),
                TheWidgetState::Selected,
            ),
            "Rect Tool" => {
                ctx.ui
                    .set_widget_state("LeftMode::Rect Tool".to_string(), TheWidgetState::Selected)
            }
            _ => {}
        }

        self.left_toolbar_active_tool = Some(tool_name.to_string());
    }

    fn populate_left_tool_name_layout(&self, layout: &mut dyn TheTextLayoutTrait) {
        fn add_left_text_command(
            layout: &mut dyn TheTextLayoutTrait,
            left_label: &str,
            id: &str,
            status: &str,
            icon_name: &str,
            _selectable: bool,
        ) {
            let mut button = TheTraybarButton::new(TheId::named(id));
            button.set_icon_name(icon_name.to_string());
            button.set_text(left_label.to_string());
            button.set_status_text(status);
            layout.add_pair(" ".to_string(), Box::new(button));
        }

        fn add_section_toggle(
            layout: &mut dyn TheTextLayoutTrait,
            title: &str,
            toggle_id: &str,
            expanded: bool,
            status: &str,
        ) {
            let mut button = TheMenubarButton::new(TheId::named(toggle_id));
            button.set_icon_name("list".to_string());
            button.set_status_text(status);
            button.set_has_state(false);
            layout.add_pair(
                format!(
                    "{} {}",
                    if expanded { "Section v" } else { "Section >" },
                    title
                ),
                Box::new(button),
            );
        }

        fn add_left_action(
            layout: &mut dyn TheTextLayoutTrait,
            section: &str,
            label: &str,
            id: &str,
            status: &str,
        ) {
            add_left_text_command(
                layout,
                &format!("{} > {}", section, label),
                id,
                status,
                match section {
                    "File" => "folder",
                    "Edit" => "pencil",
                    "View" => "eye",
                    "Select" => "cursor",
                    "Actor" => "cube",
                    "Blueprint" => "flow",
                    "Cinematics" => "video",
                    "Modes" => "layout",
                    "Platforms" => "box",
                    "Layouts" => "window",
                    "Debug" => "bug",
                    "Source" => "git-branch",
                    "Tools" => "wrench",
                    "Build" => "play",
                    "Settings" => "gear",
                    "Window" => "window",
                    "Help" => "question-mark",
                    _ => "arrow-right",
                },
                false,
            );
        }

        add_left_text_command(
            layout,
            "Sections > Expand All",
            "LeftGroupAction::ExpandAll",
            "Expand all left sections",
            "plus",
            false,
        );
        add_left_text_command(
            layout,
            "Sections > Collapse All",
            "LeftGroupAction::CollapseAll",
            "Collapse all left sections",
            "minus",
            false,
        );

        add_section_toggle(
            layout,
            "Tools & Systems",
            "LeftGroupToggle::Modes",
            self.left_group_modes_expanded,
            "Toggle tools and generation submenu",
        );
        if self.left_group_modes_expanded {
            if let Ok(toollist) = TOOLLIST.read() {
                for tool in &toollist.game_tools {
                    let tool_name = tool.id().name.clone();
                    let tool_label = self.tool_group_label(&tool_name, &tool.info());
                    let left_tool_id = format!("LeftTool::{}", tool_name);
                    add_left_text_command(
                        layout,
                        &format!("Tool > {}", tool_label),
                        &left_tool_id,
                        &format!("Activate {}", tool.info()),
                        tool.icon_name().as_str(),
                        true,
                    );
                }
            }

            let tool_actions = [
                (
                    "Town > Generate",
                    "LeftAction::MenuTools::GenerateTown",
                    "Generate town systems",
                ),
                (
                    "Town > Preset > Small Town",
                    "LeftAction::MenuTownPreset::SmallTown",
                    "Set town preset to Small Town",
                ),
                (
                    "Town > Preset > Large Town",
                    "LeftAction::MenuTownPreset::LargeTown",
                    "Set town preset to Large Town",
                ),
                (
                    "Town > Preset > Small City",
                    "LeftAction::MenuTownPreset::SmallCity",
                    "Set town preset to Small City",
                ),
                (
                    "Town > Preset > Large City",
                    "LeftAction::MenuTownPreset::LargeCity",
                    "Set town preset to Large City",
                ),
                (
                    "Town > Reseed + Generate",
                    "LeftAction::MenuTown::ReseedGenerate",
                    "Generate a town with a new random seed",
                ),
                (
                    "Town > Regenerate",
                    "LeftAction::MenuTown::Regenerate",
                    "Regenerate town with current seed",
                ),
                (
                    "Town > Toggle River",
                    "LeftAction::MenuTown::ToggleRiver",
                    "Toggle river generation",
                ),
                (
                    "Town > Toggle Walls",
                    "LeftAction::MenuTown::ToggleWalls",
                    "Toggle wall generation",
                ),
                (
                    "Town > Bake To Map",
                    "LeftAction::MenuTown::BakeMap",
                    "Bake generated town into the current map",
                ),
                (
                    "Fantasy World > Generate",
                    "LeftAction::MenuTools::GenerateFantasyWorld",
                    "Generate continents, countries, and capitals",
                ),
                (
                    "RPG/MMORPG > Generate",
                    "LeftAction::MenuTools::GenerateRpgMmorpg",
                    "Generate RPG/MMORPG systems",
                ),
                (
                    "RPG/MMORPG > Builder > Open",
                    "LeftAction::MenuMmorpgBuilder::Open",
                    "Open RPG/MMORPG builder summary",
                ),
                (
                    "RPG/MMORPG > Builder > Generate",
                    "LeftAction::MenuMmorpgBuilder::Generate",
                    "Generate RPG/MMORPG from builder inputs",
                ),
                (
                    "RPG/MMORPG > Sim > Tick",
                    "LeftAction::MenuMmoSim::Tick",
                    "Run one MMO simulation tick",
                ),
                (
                    "RPG/MMORPG > Sim > Combat",
                    "LeftAction::MenuMmoSim::Combat",
                    "Run one MMO combat simulation step",
                ),
                (
                    "RPG/MMORPG > Sim > Loot",
                    "LeftAction::MenuMmoSim::Loot",
                    "Run one MMO loot simulation step",
                ),
                (
                    "Map Forge > Open Editor",
                    "LeftAction::MenuMapForge::OpenEditor",
                    "Open Map Forge editor mode",
                ),
                (
                    "Map Forge > Tools > Select",
                    "LeftAction::MenuMapForge::ToolSelect",
                    "Use map selection tool",
                ),
                (
                    "Map Forge > Tools > Terrain",
                    "LeftAction::MenuMapForge::ToolTerrain",
                    "Use terrain/world sculpt tool",
                ),
                (
                    "Map Forge > Tools > Roads",
                    "LeftAction::MenuMapForge::ToolRoad",
                    "Use linedef road editing tool",
                ),
                (
                    "Map Forge > Tools > Districts",
                    "LeftAction::MenuMapForge::ToolDistrict",
                    "Use sector district editing tool",
                ),
                (
                    "Map Forge > Tools > Landmarks",
                    "LeftAction::MenuMapForge::ToolLandmark",
                    "Use entity landmark placement tool",
                ),
                (
                    "Map Forge > Layers > Districts",
                    "LeftAction::MenuMapForge::LayerDistricts",
                    "Toggle district overlay visibility",
                ),
                (
                    "Map Forge > Layers > Roads",
                    "LeftAction::MenuMapForge::LayerRoads",
                    "Toggle road overlay visibility",
                ),
                (
                    "Map Forge > Layers > Landmarks",
                    "LeftAction::MenuMapForge::LayerLandmarks",
                    "Toggle landmark overlay visibility",
                ),
                (
                    "Map Forge > Generate > Town",
                    "LeftAction::MenuMapForge::GenerateTown",
                    "Generate a town map",
                ),
                (
                    "Map Forge > Generate > Fantasy World",
                    "LeftAction::MenuMapForge::GenerateFantasyWorld",
                    "Generate a fantasy world map",
                ),
                (
                    "Map Forge > Generate > Reseed Town",
                    "LeftAction::MenuMapForge::ReseedTown",
                    "Generate town with a new seed",
                ),
                (
                    "Map Forge > Generate > Regenerate Town",
                    "LeftAction::MenuMapForge::RegenerateTown",
                    "Regenerate town with current seed",
                ),
                (
                    "Map Forge > IO > Export JSON",
                    "LeftAction::MenuMapForge::ExportJson",
                    "Export generated map JSON",
                ),
                (
                    "Map Forge > IO > Import JSON",
                    "LeftAction::MenuMapForge::ImportJson",
                    "Import map JSON",
                ),
                (
                    "Map Forge > IO > Bake To Map",
                    "LeftAction::MenuMapForge::BakeMap",
                    "Bake generated town data into current map",
                ),
            ];
            for (label, id, status) in tool_actions {
                add_left_action(layout, "Tools", label, id, status);
            }
        }

        add_section_toggle(
            layout,
            "Project & Edit",
            "LeftGroupToggle::2D",
            self.left_group_2d_expanded,
            "Toggle file/edit/view submenu",
        );
        if self.left_group_2d_expanded {
            let project_actions = [
                ("File", "New", "LeftAction::MenuFile::New", "Create a new project"),
                (
                    "File",
                    "Open",
                    "LeftAction::MenuFile::Open",
                    "Open an existing project",
                ),
                ("File", "Save", "LeftAction::MenuFile::Save", "Save current project"),
                (
                    "File",
                    "Save As",
                    "LeftAction::MenuFile::SaveAs",
                    "Save current project under a new name",
                ),
                (
                    "File",
                    "Close Project",
                    "LeftAction::MenuFile::Close",
                    "Close current project",
                ),
                ("Edit", "Undo", "LeftAction::MenuEdit::Undo", "Undo last change"),
                ("Edit", "Redo", "LeftAction::MenuEdit::Redo", "Redo last undone change"),
                (
                    "Edit",
                    "Copy",
                    "LeftAction::MenuEdit::Copy",
                    "Copy current selection",
                ),
                (
                    "Edit",
                    "Paste",
                    "LeftAction::MenuEdit::Paste",
                    "Paste from clipboard",
                ),
                (
                    "View",
                    "Toggle Left Toolbar",
                    "LeftAction::MenuView::ToggleLeft",
                    "Toggle left toolbar visibility",
                ),
                (
                    "View",
                    "Toggle Right Toolbar",
                    "LeftAction::MenuView::ToggleRight",
                    "Toggle right toolbar visibility",
                ),
                (
                    "View",
                    "Theme > Dark",
                    "LeftAction::MenuTheme::Dark",
                    "Apply dark theme",
                ),
                (
                    "View",
                    "Theme > Light",
                    "LeftAction::MenuTheme::Light",
                    "Apply light theme",
                ),
                (
                    "View",
                    "Theme > Slate",
                    "LeftAction::MenuTheme::Slate",
                    "Apply slate theme",
                ),
                (
                    "Select",
                    "Select All",
                    "LeftAction::MenuSelect::All",
                    "Select all relevant map content",
                ),
                (
                    "Select",
                    "Select None",
                    "LeftAction::MenuSelect::None",
                    "Clear current selection",
                ),
                (
                    "Actor",
                    "Place > Entity",
                    "LeftAction::MenuActor::PlaceEntity",
                    "Switch to entity placement tool",
                ),
                (
                    "Actor",
                    "Snap To Grid",
                    "LeftAction::MenuActor::SnapToGrid",
                    "Toggle actor snapping behavior",
                ),
                (
                    "Blueprint",
                    "Open Blueprint Panel",
                    "LeftAction::MenuBlueprint::OpenPanel",
                    "Open blueprint panel snapshot",
                ),
                (
                    "Cinematics",
                    "Create Level Sequence",
                    "LeftAction::MenuCinematics::CreateSequence",
                    "Create a level sequence scaffold",
                ),
                (
                    "Modes",
                    "Select",
                    "LeftAction::MenuMode::Select",
                    "Switch to selection mode",
                ),
                (
                    "Modes",
                    "Landscape",
                    "LeftAction::MenuMode::Landscape",
                    "Switch to landscape editing mode",
                ),
                (
                    "Modes",
                    "Foliage",
                    "LeftAction::MenuMode::Foliage",
                    "Switch to foliage/entity painting mode",
                ),
                (
                    "Modes",
                    "Game View",
                    "LeftAction::MenuMode::GameView",
                    "Toggle game input mode",
                ),
                (
                    "Platforms",
                    "Package > Windows",
                    "LeftAction::MenuPlatforms::PackageWindows",
                    "Export/package for Windows",
                ),
                (
                    "Platforms",
                    "Package > Linux",
                    "LeftAction::MenuPlatforms::PackageLinux",
                    "Export/package for Linux",
                ),
                (
                    "Platforms",
                    "Package > Web",
                    "LeftAction::MenuPlatforms::PackageWeb",
                    "Export/package for Web",
                ),
                (
                    "Layouts",
                    "Window > Unreal",
                    "LeftAction::MenuLayouts::Unreal",
                    "Apply Unreal-like layout preset",
                ),
                (
                    "Layouts",
                    "Window > Minimal",
                    "LeftAction::MenuLayouts::Minimal",
                    "Apply minimal layout preset",
                ),
                (
                    "Debug",
                    "Toggle Runtime",
                    "LeftAction::MenuDebug::ToggleRuntime",
                    "Toggle game runtime play/stop",
                ),
                (
                    "Debug",
                    "Show Feature Matrix",
                    "LeftAction::MenuDebug::FeatureMatrix",
                    "Open IDE feature matrix",
                ),
                (
                    "Source",
                    "Submit Content",
                    "LeftAction::MenuSource::SubmitContent",
                    "Open source control submit flow",
                ),
            ];
            for (section, label, id, status) in project_actions {
                add_left_action(layout, section, label, id, status);
            }
        }

        add_section_toggle(
            layout,
            "Build & Settings",
            "LeftGroupToggle::3D",
            self.left_group_3d_expanded,
            "Toggle build/settings submenu",
        );
        if self.left_group_3d_expanded {
            let runtime_actions = [
                ("Build", "Play", "LeftAction::MenuBuild::Play", "Run the game"),
                ("Build", "Pause", "LeftAction::MenuBuild::Pause", "Pause runtime"),
                ("Build", "Stop", "LeftAction::MenuBuild::Stop", "Stop runtime"),
                (
                    "Build",
                    "Export 2D",
                    "LeftAction::MenuBuild::Export2D",
                    "Export 2D package",
                ),
                (
                    "Build",
                    "Export 3D",
                    "LeftAction::MenuBuild::Export3D",
                    "Export 3D package",
                ),
                (
                    "Settings",
                    "Toggle Snap",
                    "LeftAction::MenuOption::Snap",
                    "Toggle snap-to-grid option",
                ),
                (
                    "Settings",
                    "Toggle Grid",
                    "LeftAction::MenuOption::Grid",
                    "Toggle grid visibility option",
                ),
                (
                    "Settings",
                    "Toggle Gizmos",
                    "LeftAction::MenuOption::Gizmos",
                    "Toggle gizmo visibility option",
                ),
                (
                    "Settings",
                    "IDE Layout > Unreal",
                    "LeftAction::MenuIde::LayoutUnreal",
                    "Apply Unreal-like IDE layout preset",
                ),
                (
                    "Settings",
                    "IDE Layout > Minimal",
                    "LeftAction::MenuIde::LayoutMinimal",
                    "Apply minimal IDE layout preset",
                ),
                (
                    "Settings",
                    "IDE Feature Matrix",
                    "LeftAction::MenuIde::FeatureMatrix",
                    "Open IDE feature matrix",
                ),
            ];
            for (section, label, id, status) in runtime_actions {
                add_left_action(layout, section, label, id, status);
            }
        }

        add_section_toggle(
            layout,
            "Panels & Help",
            "LeftGroupToggle::Editor",
            self.left_group_editor_expanded,
            "Toggle window/help submenu",
        );
        if self.left_group_editor_expanded {
            let panel_actions = [
                (
                    "Window",
                    "Content Browser",
                    "LeftAction::MenuWindow::ContentBrowser",
                    "Open content browser panel",
                ),
                (
                    "Window",
                    "World Outliner",
                    "LeftAction::MenuWindow::WorldOutliner",
                    "Open world outliner panel",
                ),
                (
                    "Window",
                    "Details",
                    "LeftAction::MenuWindow::Details",
                    "Open details panel",
                ),
                (
                    "Window",
                    "Output Log",
                    "LeftAction::MenuWindow::OutputLog",
                    "Open output log panel",
                ),
                (
                    "Window",
                    "Blueprint",
                    "LeftAction::MenuWindow::Blueprint",
                    "Open blueprint panel",
                ),
                (
                    "Help",
                    "Docs",
                    "LeftAction::MenuHelp::Docs",
                    "Open help documentation",
                ),
                (
                    "Help",
                    "Examples",
                    "LeftAction::MenuHelp::Examples",
                    "Open examples page",
                ),
                (
                    "Help",
                    "About",
                    "LeftAction::MenuHelp::About",
                    "Open app about dialog",
                ),
            ];
            for (section, label, id, status) in panel_actions {
                add_left_action(layout, section, label, id, status);
            }
        }
    }

    fn rebuild_left_tool_name_layout(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        if let Some(layout) = ui.get_text_layout("Left Tool Name Layout") {
            layout.clear();
            self.populate_left_tool_name_layout(layout);
            layout.relayout(ctx);
        }
        self.left_toolbar_active_tool = None;
        self.sync_left_toolbar_active_from_toollist(ctx);
    }

    fn sync_left_toolbar_active_from_toollist(&mut self, ctx: &mut TheContext) {
        if let Ok(toollist) = TOOLLIST.read()
            && !toollist.editor_mode
            && let Some(tool) = toollist.game_tools.get(toollist.curr_game_tool)
        {
            self.set_left_toolbar_active_tool(ctx, &tool.id().name);
        }
    }

    fn persist_workspace_settings_to_project_config(&mut self) {
        let mut root_value = toml::from_str::<toml::Value>(&self.project.config)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let root = if let Some(root) = root_value.as_table_mut() {
            root
        } else {
            root_value = toml::Value::Table(toml::Table::new());
            root_value.as_table_mut().unwrap()
        };

        let layout_entry = root
            .entry("editor_layout".to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if let Some(layout) = layout_entry.as_table_mut() {
            layout.insert(
                "show_left_toolbar".to_string(),
                toml::Value::Boolean(self.show_left_toolbar),
            );
            layout.insert(
                "show_right_toolbar".to_string(),
                toml::Value::Boolean(self.show_right_toolbar),
            );
            layout.insert(
                "option_snap_to_grid".to_string(),
                toml::Value::Boolean(self.option_snap_to_grid),
            );
            layout.insert(
                "option_show_grid".to_string(),
                toml::Value::Boolean(self.option_show_grid),
            );
            layout.insert(
                "option_show_gizmos".to_string(),
                toml::Value::Boolean(self.option_show_gizmos),
            );
            layout.insert(
                "theme_preset".to_string(),
                toml::Value::String(self.theme_preset.clone()),
            );
            layout.insert(
                "left_group_modes_expanded".to_string(),
                toml::Value::Boolean(self.left_group_modes_expanded),
            );
            layout.insert(
                "left_group_2d_expanded".to_string(),
                toml::Value::Boolean(self.left_group_2d_expanded),
            );
            layout.insert(
                "left_group_3d_expanded".to_string(),
                toml::Value::Boolean(self.left_group_3d_expanded),
            );
            layout.insert(
                "left_group_editor_expanded".to_string(),
                toml::Value::Boolean(self.left_group_editor_expanded),
            );
            layout.insert(
                "towngen_preset".to_string(),
                toml::Value::String(self.towngen_preset.clone()),
            );
            layout.insert(
                "towngen_has_river".to_string(),
                toml::Value::Boolean(self.towngen_has_river),
            );
            layout.insert(
                "towngen_has_walls".to_string(),
                toml::Value::Boolean(self.towngen_has_walls),
            );
            layout.insert(
                "towngen_last_seed".to_string(),
                toml::Value::Integer(self.towngen_last_seed as i64),
            );
            layout.insert(
                "towngen_auto_bake".to_string(),
                toml::Value::Boolean(self.towngen_auto_bake),
            );
            layout.insert(
                "overlay_show_town_districts".to_string(),
                toml::Value::Boolean(self.overlay_show_town_districts),
            );
            layout.insert(
                "overlay_show_town_roads".to_string(),
                toml::Value::Boolean(self.overlay_show_town_roads),
            );
            layout.insert(
                "overlay_show_town_landmarks".to_string(),
                toml::Value::Boolean(self.overlay_show_town_landmarks),
            );
            layout.insert(
                "mmorpg_xp_rate".to_string(),
                toml::Value::Float(self.mmorpg_xp_rate as f64),
            );
            layout.insert(
                "mmorpg_loot_rate".to_string(),
                toml::Value::Float(self.mmorpg_loot_rate as f64),
            );
            layout.insert(
                "mmorpg_event_rate".to_string(),
                toml::Value::Float(self.mmorpg_event_rate as f64),
            );
            layout.insert(
                "mmorpg_world_name".to_string(),
                toml::Value::String(self.mmorpg_world_name.clone()),
            );
            layout.insert(
                "mmorpg_max_players".to_string(),
                toml::Value::Integer(self.mmorpg_max_players as i64),
            );
            layout.insert(
                "mmorpg_starting_level".to_string(),
                toml::Value::Integer(self.mmorpg_starting_level as i64),
            );
            layout.insert(
                "mmorpg_race_count".to_string(),
                toml::Value::Integer(self.mmorpg_race_count as i64),
            );
            layout.insert(
                "mmorpg_quest_count".to_string(),
                toml::Value::Integer(self.mmorpg_quest_count as i64),
            );
            layout.insert(
                "mmorpg_skill_tier_count".to_string(),
                toml::Value::Integer(self.mmorpg_skill_tier_count as i64),
            );
            layout.insert(
                "mmorpg_include_warrior".to_string(),
                toml::Value::Boolean(self.mmorpg_include_warrior),
            );
            layout.insert(
                "mmorpg_include_ranger".to_string(),
                toml::Value::Boolean(self.mmorpg_include_ranger),
            );
            layout.insert(
                "mmorpg_include_mage".to_string(),
                toml::Value::Boolean(self.mmorpg_include_mage),
            );
            layout.insert(
                "mmorpg_include_cleric".to_string(),
                toml::Value::Boolean(self.mmorpg_include_cleric),
            );
            layout.insert(
                "mmorpg_include_rogue".to_string(),
                toml::Value::Boolean(self.mmorpg_include_rogue),
            );
            layout.insert(
                "fantasy_world_name".to_string(),
                toml::Value::String(self.fantasy_world_name.clone()),
            );
            layout.insert(
                "fantasy_world_seed".to_string(),
                toml::Value::Integer(self.fantasy_world_seed as i64),
            );
            layout.insert(
                "fantasy_continent_count".to_string(),
                toml::Value::Integer(self.fantasy_continent_count as i64),
            );
            layout.insert(
                "fantasy_countries_per_continent".to_string(),
                toml::Value::Integer(self.fantasy_countries_per_continent as i64),
            );
            layout.insert(
                "fantasy_towns_per_country".to_string(),
                toml::Value::Integer(self.fantasy_towns_per_country as i64),
            );
            layout.insert(
                "fantasy_has_islands".to_string(),
                toml::Value::Boolean(self.fantasy_has_islands),
            );
        }

        if let Ok(config_text) = toml::to_string_pretty(&root_value) {
            self.project.config = config_text;
        }
    }

    fn load_workspace_settings_from_project_config(&mut self) {
        if let Ok(root_value) = toml::from_str::<toml::Value>(&self.project.config)
            && let Some(layout) = root_value.get("editor_layout").and_then(toml::Value::as_table)
        {
            if let Some(v) = layout.get("show_left_toolbar").and_then(toml::Value::as_bool) {
                self.show_left_toolbar = v;
            }
            if let Some(v) = layout.get("show_right_toolbar").and_then(toml::Value::as_bool) {
                self.show_right_toolbar = v;
            }
            if let Some(v) = layout.get("option_snap_to_grid").and_then(toml::Value::as_bool) {
                self.option_snap_to_grid = v;
            }
            if let Some(v) = layout.get("option_show_grid").and_then(toml::Value::as_bool) {
                self.option_show_grid = v;
            }
            if let Some(v) = layout.get("option_show_gizmos").and_then(toml::Value::as_bool) {
                self.option_show_gizmos = v;
            }
            if let Some(v) = layout.get("theme_preset").and_then(toml::Value::as_str) {
                self.theme_preset = v.to_string();
            }
            if let Some(v) = layout
                .get("left_group_modes_expanded")
                .and_then(toml::Value::as_bool)
            {
                self.left_group_modes_expanded = v;
            }
            if let Some(v) = layout
                .get("left_group_2d_expanded")
                .and_then(toml::Value::as_bool)
            {
                self.left_group_2d_expanded = v;
            }
            if let Some(v) = layout
                .get("left_group_3d_expanded")
                .and_then(toml::Value::as_bool)
            {
                self.left_group_3d_expanded = v;
            }
            if let Some(v) = layout
                .get("left_group_editor_expanded")
                .and_then(toml::Value::as_bool)
            {
                self.left_group_editor_expanded = v;
            }
            if let Some(v) = layout.get("towngen_preset").and_then(toml::Value::as_str) {
                self.towngen_preset = v.to_string();
            }
            if let Some(v) = layout.get("towngen_has_river").and_then(toml::Value::as_bool) {
                self.towngen_has_river = v;
            }
            if let Some(v) = layout.get("towngen_has_walls").and_then(toml::Value::as_bool) {
                self.towngen_has_walls = v;
            }
            if let Some(v) = layout.get("towngen_last_seed").and_then(toml::Value::as_integer) {
                self.towngen_last_seed = v.max(1) as u64;
            }
            if let Some(v) = layout.get("towngen_auto_bake").and_then(toml::Value::as_bool) {
                self.towngen_auto_bake = v;
            }
            if let Some(v) = layout
                .get("overlay_show_town_districts")
                .and_then(toml::Value::as_bool)
            {
                self.overlay_show_town_districts = v;
            }
            if let Some(v) = layout
                .get("overlay_show_town_roads")
                .and_then(toml::Value::as_bool)
            {
                self.overlay_show_town_roads = v;
            }
            if let Some(v) = layout
                .get("overlay_show_town_landmarks")
                .and_then(toml::Value::as_bool)
            {
                self.overlay_show_town_landmarks = v;
            }
            if let Some(v) = layout.get("mmorpg_xp_rate").and_then(toml::Value::as_float) {
                self.mmorpg_xp_rate = (v as f32).clamp(0.1, 5.0);
            }
            if let Some(v) = layout.get("mmorpg_loot_rate").and_then(toml::Value::as_float) {
                self.mmorpg_loot_rate = (v as f32).clamp(0.1, 5.0);
            }
            if let Some(v) = layout.get("mmorpg_event_rate").and_then(toml::Value::as_float) {
                self.mmorpg_event_rate = (v as f32).clamp(0.1, 5.0);
            }
            if let Some(v) = layout.get("mmorpg_world_name").and_then(toml::Value::as_str) {
                self.mmorpg_world_name = v.to_string();
            }
            if let Some(v) = layout.get("mmorpg_max_players").and_then(toml::Value::as_integer) {
                self.mmorpg_max_players = (v as i32).clamp(10, 100_000);
            }
            if let Some(v) = layout
                .get("mmorpg_starting_level")
                .and_then(toml::Value::as_integer)
            {
                self.mmorpg_starting_level = (v as i32).clamp(1, 99);
            }
            if let Some(v) = layout.get("mmorpg_race_count").and_then(toml::Value::as_integer) {
                self.mmorpg_race_count = (v as i32).clamp(1, 12);
            }
            if let Some(v) = layout.get("mmorpg_quest_count").and_then(toml::Value::as_integer) {
                self.mmorpg_quest_count = (v as i32).clamp(1, 64);
            }
            if let Some(v) = layout
                .get("mmorpg_skill_tier_count")
                .and_then(toml::Value::as_integer)
            {
                self.mmorpg_skill_tier_count = (v as i32).clamp(1, 6);
            }
            if let Some(v) = layout
                .get("mmorpg_include_warrior")
                .and_then(toml::Value::as_bool)
            {
                self.mmorpg_include_warrior = v;
            }
            if let Some(v) = layout
                .get("mmorpg_include_ranger")
                .and_then(toml::Value::as_bool)
            {
                self.mmorpg_include_ranger = v;
            }
            if let Some(v) = layout
                .get("mmorpg_include_mage")
                .and_then(toml::Value::as_bool)
            {
                self.mmorpg_include_mage = v;
            }
            if let Some(v) = layout
                .get("mmorpg_include_cleric")
                .and_then(toml::Value::as_bool)
            {
                self.mmorpg_include_cleric = v;
            }
            if let Some(v) = layout
                .get("mmorpg_include_rogue")
                .and_then(toml::Value::as_bool)
            {
                self.mmorpg_include_rogue = v;
            }
            if let Some(v) = layout.get("fantasy_world_name").and_then(toml::Value::as_str) {
                self.fantasy_world_name = v.to_string();
            }
            if let Some(v) = layout
                .get("fantasy_world_seed")
                .and_then(toml::Value::as_integer)
            {
                self.fantasy_world_seed = v.max(0) as u64;
            }
            if let Some(v) = layout
                .get("fantasy_continent_count")
                .and_then(toml::Value::as_integer)
            {
                self.fantasy_continent_count = (v as i32).clamp(1, 12);
            }
            if let Some(v) = layout
                .get("fantasy_countries_per_continent")
                .and_then(toml::Value::as_integer)
            {
                self.fantasy_countries_per_continent = (v as i32).clamp(1, 24);
            }
            if let Some(v) = layout
                .get("fantasy_towns_per_country")
                .and_then(toml::Value::as_integer)
            {
                self.fantasy_towns_per_country = (v as i32).clamp(1, 20);
            }
            if let Some(v) = layout
                .get("fantasy_has_islands")
                .and_then(toml::Value::as_bool)
            {
                self.fantasy_has_islands = v;
            }
        }
    }

    fn apply_workspace_settings_to_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
        self.server_ctx.show_editing_geometry = self.option_show_gizmos;

        ui.set_widget_value(
            "OptionLeftToolbarCB",
            ctx,
            TheValue::Bool(self.show_left_toolbar),
        );
        ui.set_widget_value(
            "OptionRightToolbarCB",
            ctx,
            TheValue::Bool(self.show_right_toolbar),
        );
        ui.set_widget_value("OptionSnapCB", ctx, TheValue::Bool(self.option_snap_to_grid));
        ui.set_widget_value("OptionGridCB", ctx, TheValue::Bool(self.option_show_grid));
        ui.set_widget_value("OptionGizmoCB", ctx, TheValue::Bool(self.option_show_gizmos));
        ui.set_widget_value(
            "TownSeedEdit",
            ctx,
            TheValue::Int(self.towngen_last_seed as i32),
        );
        ui.set_widget_value("TownRiverCB", ctx, TheValue::Bool(self.towngen_has_river));
        ui.set_widget_value("TownWallsCB", ctx, TheValue::Bool(self.towngen_has_walls));
        ui.set_widget_value("TownAutoBakeCB", ctx, TheValue::Bool(self.towngen_auto_bake));
        ui.set_widget_value(
            "OverlayTownDistrictsCB",
            ctx,
            TheValue::Bool(self.overlay_show_town_districts),
        );
        ui.set_widget_value(
            "OverlayTownRoadsCB",
            ctx,
            TheValue::Bool(self.overlay_show_town_roads),
        );
        ui.set_widget_value(
            "OverlayTownLandmarksCB",
            ctx,
            TheValue::Bool(self.overlay_show_town_landmarks),
        );
        ui.set_widget_value("MmorpgXpRateEdit", ctx, TheValue::Float(self.mmorpg_xp_rate));
        ui.set_widget_value("MmorpgLootRateEdit", ctx, TheValue::Float(self.mmorpg_loot_rate));
        ui.set_widget_value("MmorpgEventRateEdit", ctx, TheValue::Float(self.mmorpg_event_rate));
        ui.set_widget_value(
            "MmorpgWorldNameEdit",
            ctx,
            TheValue::Text(self.mmorpg_world_name.clone()),
        );
        ui.set_widget_value(
            "MmorpgMaxPlayersEdit",
            ctx,
            TheValue::Int(self.mmorpg_max_players),
        );
        ui.set_widget_value(
            "MmorpgStartingLevelEdit",
            ctx,
            TheValue::Int(self.mmorpg_starting_level),
        );
        ui.set_widget_value("MmorpgRaceCountEdit", ctx, TheValue::Int(self.mmorpg_race_count));
        ui.set_widget_value(
            "MmorpgQuestCountEdit",
            ctx,
            TheValue::Int(self.mmorpg_quest_count),
        );
        ui.set_widget_value(
            "MmorpgSkillTierCountEdit",
            ctx,
            TheValue::Int(self.mmorpg_skill_tier_count),
        );
        ui.set_widget_value(
            "MmorpgClassWarriorCB",
            ctx,
            TheValue::Bool(self.mmorpg_include_warrior),
        );
        ui.set_widget_value(
            "MmorpgClassRangerCB",
            ctx,
            TheValue::Bool(self.mmorpg_include_ranger),
        );
        ui.set_widget_value(
            "MmorpgClassMageCB",
            ctx,
            TheValue::Bool(self.mmorpg_include_mage),
        );
        ui.set_widget_value(
            "MmorpgClassClericCB",
            ctx,
            TheValue::Bool(self.mmorpg_include_cleric),
        );
        ui.set_widget_value(
            "MmorpgClassRogueCB",
            ctx,
            TheValue::Bool(self.mmorpg_include_rogue),
        );
        ui.set_widget_value(
            "FantasyWorldNameEdit",
            ctx,
            TheValue::Text(self.fantasy_world_name.clone()),
        );
        ui.set_widget_value(
            "FantasyWorldSeedEdit",
            ctx,
            TheValue::Int(self.fantasy_world_seed as i32),
        );
        ui.set_widget_value(
            "FantasyContinentCountEdit",
            ctx,
            TheValue::Int(self.fantasy_continent_count),
        );
        ui.set_widget_value(
            "FantasyCountriesPerContinentEdit",
            ctx,
            TheValue::Int(self.fantasy_countries_per_continent),
        );
        ui.set_widget_value(
            "FantasyTownsPerCountryEdit",
            ctx,
            TheValue::Int(self.fantasy_towns_per_country),
        );
        ui.set_widget_value(
            "FantasyHasIslandsCB",
            ctx,
            TheValue::Bool(self.fantasy_has_islands),
        );

        let theme_index = match self.theme_preset.as_str() {
            "Light" => 1,
            "Slate" => 2,
            _ => 0,
        };
        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(theme_index));
        let town_preset_index = match self.towngen_preset.as_str() {
            "Small Town" => 0,
            "Large Town" => 1,
            "Large City" => 3,
            _ => 2,
        };
        ui.set_widget_value("TownPresetDropdown", ctx, TheValue::Int(town_preset_index));

        self.rebuild_left_tool_name_layout(ui, ctx);
        self.apply_toolbar_visibility(ui, ctx);
        self.apply_theme_preset(ui, ctx);
        self.apply_town_overlay_visibility();
    }

    fn generate_town_system_data(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(1);
        self.towngen_last_seed = seed;
        self.generate_town_system_data_with_seed(ui, ctx, seed);
    }

    fn town_preset_to_settings(&self, seed: u64) -> crate::game_logic::TownGeneratorSettings {
        let (size, rings, districts) = match self.towngen_preset.as_str() {
            "Small Town" => (720, 3, 5),
            "Large Town" => (860, 4, 6),
            "Small City" => (1024, 4, 7),
            "Large City" => (1280, 5, 8),
            _ => (1024, 4, 7),
        };

        crate::game_logic::TownGeneratorSettings {
            seed,
            town_name: if self.project.name.is_empty() {
                "Procedural Town".to_string()
            } else {
                format!("{} Town", self.project.name)
            },
            size,
            rings,
            districts_per_ring: districts,
            has_river: self.towngen_has_river,
            has_walls: self.towngen_has_walls,
        }
    }

    fn generate_town_system_data_with_seed(
        &mut self,
        ui: &mut TheUI,
        ctx: &mut TheContext,
        seed: u64,
    ) {
        self.towngen_last_seed = seed;

        let settings = self.town_preset_to_settings(seed);

        let generated = crate::game_logic::generate_town_map(&settings);
        self.last_generated_town = Some(generated.clone());
        let payload = serde_json::to_string_pretty(&generated).unwrap_or_else(|_| "{}".to_string());

        let mut root_value = toml::from_str::<toml::Value>(&self.project.config)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let root = if let Some(root) = root_value.as_table_mut() {
            root
        } else {
            root_value = toml::Value::Table(toml::Table::new());
            root_value.as_table_mut().unwrap()
        };

        let tg_entry = root
            .entry("town_generator".to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if let Some(tg) = tg_entry.as_table_mut() {
            tg.insert(
                "source".to_string(),
                toml::Value::String(
                    "Inspired by watabou/MapForge (formerly TownGeneratorOS), adapted for Enchentment Engine"
                        .to_string(),
                ),
            );
            tg.insert(
                "source_repo".to_string(),
                toml::Value::String("https://github.com/watabou/TownGeneratorOS".to_string()),
            );
            tg.insert(
                "preset".to_string(),
                toml::Value::String(self.towngen_preset.clone()),
            );
            tg.insert("seed".to_string(), toml::Value::Integer(seed as i64));
            tg.insert(
                "has_river".to_string(),
                toml::Value::Boolean(self.towngen_has_river),
            );
            tg.insert(
                "has_walls".to_string(),
                toml::Value::Boolean(self.towngen_has_walls),
            );
            tg.insert(
                "district_count".to_string(),
                toml::Value::Integer(generated.districts.len() as i64),
            );
            tg.insert(
                "road_count".to_string(),
                toml::Value::Integer(generated.roads.len() as i64),
            );
            tg.insert(
                "landmark_count".to_string(),
                toml::Value::Integer(generated.landmarks.len() as i64),
            );
            tg.insert("last_generated".to_string(), toml::Value::String(payload));
        }

        if let Ok(config_text) = toml::to_string_pretty(&root_value) {
            self.project.config = config_text;
        }

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(620, 240));
        let mut layout = TheTextLayout::new(TheId::named("Town Generator Result"));
        layout.set_margin(Vec4::new(12, 12, 12, 12));
        layout.set_padding(6);

        let mut txt = TheText::new(TheId::named("Town Generator Result Text"));
        txt.set_text(format!(
            "Town generated and embedded into project config.\n\
             Preset: {}\n\
             Seed: {}\n\
             River: {}\n\
             Walls: {}\n\
             Districts: {}\n\
             Roads: {}\n\
             Landmarks: {}\n\
             Config key: [town_generator]",
            self.towngen_preset,
            generated.seed,
            self.towngen_has_river,
            self.towngen_has_walls,
            generated.districts.len(),
            generated.roads.len(),
            generated.landmarks.len()
        ));
        layout.add_pair("".to_string(), Box::new(txt));
        canvas.set_layout(layout);
        ui.show_dialog(
            "Town Generator",
            canvas,
            vec![TheDialogButtonRole::Accept],
            ctx,
        );

        ctx.ui.send(TheEvent::SetStatusText(
            TheId::empty(),
            "Town generator data created and stored in project config.".to_string(),
        ));
        self.persist_workspace_settings_to_project_config();

        if self.towngen_auto_bake {
            self.bake_generated_town_to_current_map(ui, ctx);
        }
    }

    fn generate_rpg_mmorpg_system_data(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(1);
        let fallback_world = if self.project.name.is_empty() {
            "Encheament Online".to_string()
        } else {
            format!("{} Online", self.project.name)
        };
        let world_name = if self.mmorpg_world_name.trim().is_empty() {
            fallback_world
        } else {
            self.mmorpg_world_name.trim().to_string()
        };
        self.mmorpg_world_name = world_name.clone();

        let input = crate::game_logic::RpgMmorpgCreateInput {
            world_name,
            max_players_per_shard: self.mmorpg_max_players.clamp(10, 100_000) as u32,
            starting_level: self.mmorpg_starting_level.clamp(1, 99) as u32,
            race_count: self.mmorpg_race_count.clamp(1, 12) as u32,
            quest_count: self.mmorpg_quest_count.clamp(1, 64) as u32,
            skill_tier_count: self.mmorpg_skill_tier_count.clamp(1, 6) as u32,
            include_warrior: self.mmorpg_include_warrior,
            include_ranger: self.mmorpg_include_ranger,
            include_mage: self.mmorpg_include_mage,
            include_cleric: self.mmorpg_include_cleric,
            include_rogue: self.mmorpg_include_rogue,
        };

        let mut generated =
            crate::game_logic::generate_starter_rpg_mmorpg_config_with_input(seed, &input);
        generated.world_state.server_tick_ms =
            ((generated.world_state.server_tick_ms as f32) / self.mmorpg_event_rate.max(0.1))
                as u64;
        self.last_generated_mmorpg = Some(generated.clone());
        let payload = serde_json::to_string_pretty(&generated).unwrap_or_else(|_| "{}".to_string());

        let mut root_value = toml::from_str::<toml::Value>(&self.project.config)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let root = if let Some(root) = root_value.as_table_mut() {
            root
        } else {
            root_value = toml::Value::Table(toml::Table::new());
            root_value.as_table_mut().unwrap()
        };

        let systems_entry = root
            .entry("mmorpg_systems".to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if let Some(systems) = systems_entry.as_table_mut() {
            systems.insert("seed".to_string(), toml::Value::Integer(seed as i64));
            systems.insert(
                "world_name".to_string(),
                toml::Value::String(generated.world_state.world_name.clone()),
            );
            systems.insert(
                "class_count".to_string(),
                toml::Value::Integer(generated.default_classes.len() as i64),
            );
            systems.insert(
                "class_template_count".to_string(),
                toml::Value::Integer(generated.class_templates.len() as i64),
            );
            systems.insert(
                "race_count".to_string(),
                toml::Value::Integer(generated.race_templates.len() as i64),
            );
            systems.insert(
                "skill_count".to_string(),
                toml::Value::Integer(generated.starter_skills.len() as i64),
            );
            systems.insert(
                "quest_count".to_string(),
                toml::Value::Integer(generated.starter_quests.len() as i64),
            );
            systems.insert(
                "loot_entries".to_string(),
                toml::Value::Integer(generated.starter_loot_table.len() as i64),
            );
            systems.insert("data".to_string(), toml::Value::String(payload));
        }

        if let Ok(config_text) = toml::to_string_pretty(&root_value) {
            self.project.config = config_text;
        }

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(640, 260));
        let mut layout = TheTextLayout::new(TheId::named("RpgMmorpg Generator Result"));
        layout.set_margin(Vec4::new(12, 12, 12, 12));
        layout.set_padding(6);

        let mut txt = TheText::new(TheId::named("RpgMmorpg Generator Result Text"));
        txt.set_text(format!(
            "RPG/MMORPG systems generated and embedded into project config.\n\
             Seed: {}\n\
             World: {}\n\
             Classes: {}\n\
             Class Templates: {}\n\
             Race Types: {}\n\
             Skills: {}\n\
             Quests: {}\n\
             Loot Entries: {}\n\
             Config key: [mmorpg_systems]",
            generated.seed,
            generated.world_state.world_name,
            generated.default_classes.len(),
            generated.class_templates.len(),
            generated.race_templates.len(),
            generated.starter_skills.len(),
            generated.starter_quests.len(),
            generated.starter_loot_table.len()
        ));
        layout.add_pair("".to_string(), Box::new(txt));
        canvas.set_layout(layout);
        ui.show_dialog(
            "RPG/MMORPG Systems",
            canvas,
            vec![TheDialogButtonRole::Accept],
            ctx,
        );

        ctx.ui.send(TheEvent::SetStatusText(
            TheId::empty(),
            "RPG/MMORPG systems created and stored in project config.".to_string(),
        ));
    }

    fn generate_fantasy_world_system_data(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        let seed = if self.fantasy_world_seed == 0 {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(1)
        } else {
            self.fantasy_world_seed
        };
        self.fantasy_world_seed = seed;

        let world_name = if self.fantasy_world_name.trim().is_empty() {
            "Aetheria".to_string()
        } else {
            self.fantasy_world_name.trim().to_string()
        };
        self.fantasy_world_name = world_name.clone();

        let settings = crate::game_logic::FantasyMapSettings {
            seed,
            world_name,
            map_size: 4096,
            continent_count: self.fantasy_continent_count.clamp(1, 12) as u32,
            countries_per_continent: self.fantasy_countries_per_continent.clamp(1, 24) as u32,
            towns_per_country: self.fantasy_towns_per_country.clamp(1, 20) as u32,
            has_islands: self.fantasy_has_islands,
        };
        let generated = crate::game_logic::generate_fantasy_world_map(&settings);
        self.last_generated_fantasy_world = Some(generated.clone());

        let payload = serde_json::to_string_pretty(&generated).unwrap_or_else(|_| "{}".to_string());
        let mut root_value = toml::from_str::<toml::Value>(&self.project.config)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let root = if let Some(root) = root_value.as_table_mut() {
            root
        } else {
            root_value = toml::Value::Table(toml::Table::new());
            root_value.as_table_mut().unwrap()
        };
        let entry = root
            .entry("fantasy_world_generator".to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if let Some(tbl) = entry.as_table_mut() {
            tbl.insert("seed".to_string(), toml::Value::Integer(generated.seed as i64));
            tbl.insert(
                "world_name".to_string(),
                toml::Value::String(generated.world_name.clone()),
            );
            tbl.insert(
                "continent_count".to_string(),
                toml::Value::Integer(generated.continents.len() as i64),
            );
            tbl.insert(
                "country_count".to_string(),
                toml::Value::Integer(generated.countries.len() as i64),
            );
            tbl.insert(
                "border_count".to_string(),
                toml::Value::Integer(generated.borders.len() as i64),
            );
            tbl.insert("data".to_string(), toml::Value::String(payload));
        }
        if let Ok(config_text) = toml::to_string_pretty(&root_value) {
            self.project.config = config_text;
        }

        let mut canvas = TheCanvas::new();
        canvas.limiter_mut().set_max_size(Vec2::new(680, 280));
        let mut layout = TheTextLayout::new(TheId::named("Fantasy World Generator Result"));
        layout.set_margin(Vec4::new(12, 12, 12, 12));
        layout.set_padding(6);
        let mut txt = TheText::new(TheId::named("Fantasy World Generator Result Text"));
        txt.set_text(format!(
            "Fantasy world generated (MapForge-style world + town layering).\n\
             World: {}\n\
             Seed: {}\n\
             Continents: {}\n\
             Countries: {}\n\
             Borders: {}\n\
             Capital Towns: {}\n\
             Config key: [fantasy_world_generator]",
            generated.world_name,
            generated.seed,
            generated.continents.len(),
            generated.countries.len(),
            generated.borders.len(),
            generated.countries.len()
        ));
        layout.add_pair("".to_string(), Box::new(txt));
        canvas.set_layout(layout);
        ui.show_dialog(
            "Fantasy World Generator",
            canvas,
            vec![TheDialogButtonRole::Accept],
            ctx,
        );

        ctx.ui.send(TheEvent::SetStatusText(
            TheId::empty(),
            "Fantasy world generator data created and stored in project config.".to_string(),
        ));
        self.persist_workspace_settings_to_project_config();
    }

    fn apply_town_overlay_visibility(&mut self) {
        if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
            for sector in &mut map.sectors {
                if sector
                    .properties
                    .get_bool_default("town_generated_district", false)
                {
                    sector
                        .properties
                        .set("visible", Value::Bool(self.overlay_show_town_districts));
                }
            }

            for linedef in &mut map.linedefs {
                if linedef.properties.get_bool_default("town_generated_road", false) {
                    linedef
                        .properties
                        .set("visible", Value::Bool(self.overlay_show_town_roads));
                }
            }

            for item in &mut map.items {
                if item
                    .attributes
                    .get_bool_default("town_generated_landmark", false)
                {
                    item.attributes.set(
                        "visible",
                        Value::Bool(self.overlay_show_town_landmarks),
                    );
                }
            }
        }
    }

    fn bake_generated_town_to_current_map(
        &mut self,
        ui: &mut TheUI,
        ctx: &mut TheContext,
    ) -> bool {
        let Some(generated) = self.last_generated_town.clone() else {
            ctx.ui.send(TheEvent::SetStatusText(
                TheId::empty(),
                "No generated town data to bake. Generate a town first.".to_string(),
            ));
            return false;
        };

        let pc = self.server_ctx.pc;
        let mut old_map_for_undo: Option<Map> = None;
        let mut new_map_for_undo: Option<Map> = None;

        if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
            old_map_for_undo = Some(map.clone());

            let scale = 0.20f32;
            let center = generated.size as f32 * 0.5;

            for district in &generated.districts {
                let cx = (district.center.0 - center) * scale;
                let cy = (district.center.1 - center) * scale;
                let radius = district.radius * scale;
                let edges = 6usize;

                map.possible_polygon.clear();
                let mut vertex_ids = Vec::with_capacity(edges);
                for i in 0..edges {
                    let angle = (i as f32 / edges as f32) * std::f32::consts::TAU;
                    let vx = cx + angle.cos() * radius;
                    let vy = cy + angle.sin() * radius;
                    let vid = map.add_vertex_at(vx, vy);
                    vertex_ids.push(vid);
                }

                for i in 0..edges {
                    let a = vertex_ids[i];
                    let b = vertex_ids[(i + 1) % edges];
                    map.create_linedef_manual(a, b);
                }

                if let Some(sector_id) = map.close_polygon_manual()
                    && let Some(sector) = map.find_sector_mut(sector_id)
                {
                    sector
                        .properties
                        .set("town_generated_district", Value::Bool(true));
                    sector
                        .properties
                        .set("district_id", Value::Int(district.id as i32));
                    sector.properties.set(
                        "district_population",
                        Value::Int(district.population as i32),
                    );
                    sector
                        .properties
                        .set("district_wealth", Value::Float(district.wealth));
                    sector
                        .properties
                        .set("district_danger", Value::Float(district.danger));
                    sector.properties.set(
                        "district_ward",
                        Value::Str(format!("{:?}", district.ward)),
                    );
                }
            }

            for road in &generated.roads {
                let from = generated.districts.iter().find(|d| d.id == road.from);
                let to = generated.districts.iter().find(|d| d.id == road.to);
                if let (Some(a), Some(b)) = (from, to) {
                    let ax = (a.center.0 - center) * scale;
                    let ay = (a.center.1 - center) * scale;
                    let bx = (b.center.0 - center) * scale;
                    let by = (b.center.1 - center) * scale;
                    let va = map.add_vertex_at(ax, ay);
                    let vb = map.add_vertex_at(bx, by);
                    let lid = map.create_linedef_manual(va, vb);
                    map.possible_polygon.clear();
                    if let Some(linedef) = map.find_linedef_mut(lid) {
                        linedef
                            .properties
                            .set("town_generated_road", Value::Bool(true));
                        linedef
                            .properties
                            .set("road_primary", Value::Bool(road.primary));
                        linedef.properties.set("wall_width", Value::Float(0.0));
                        linedef.properties.set("wall_height", Value::Float(0.0));
                    }
                }
            }

            for landmark in &generated.landmarks {
                if let Some(district) = generated
                    .districts
                    .iter()
                    .find(|d| d.id == landmark.district_id)
                {
                    let mut poi = rusterix::Item::new();
                    poi.item_type = "town_landmark".to_string();
                    poi.attributes
                        .set("town_generated_landmark", Value::Bool(true));
                    poi.attributes
                        .set("landmark_name", Value::Str(landmark.name.clone()));
                    poi.attributes
                        .set("district_id", Value::Int(landmark.district_id as i32));
                    poi.position.x = (district.center.0 - center) * scale;
                    poi.position.z = (district.center.1 - center) * scale;
                    map.items.push(poi);
                }
            }

            map.properties
                .set("town_generated", Value::Bool(true));
            map.properties
                .set("town_generated_seed", Value::Int(generated.seed as i32));

            new_map_for_undo = Some(map.clone());
        }

        self.apply_town_overlay_visibility();

        if let (Some(old_map), Some(new_map)) = (old_map_for_undo, new_map_for_undo) {
            let atom = ProjectUndoAtom::MapEdit(pc, Box::new(old_map), Box::new(new_map));
            UNDOMANAGER.write().unwrap().add_undo(atom, ctx);
            crate::utils::scenemanager_render_map(&self.project, &self.server_ctx);
            RUSTERIX.write().unwrap().set_dirty();
            self.mapeditor.load_from_project(ui, ctx, &self.project);

            ctx.ui.send(TheEvent::SetStatusText(
                TheId::empty(),
                "Town data baked into current map (districts, roads, landmarks).".to_string(),
            ));
            return true;
        }

        false
    }

    fn generate_quests_from_last_town(&mut self) -> usize {
        let Some(town) = &self.last_generated_town else {
            return 0;
        };
        let mut added = 0usize;

        let mut root_value = toml::from_str::<toml::Value>(&self.project.config)
            .unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        let root = if let Some(root) = root_value.as_table_mut() {
            root
        } else {
            root_value = toml::Value::Table(toml::Table::new());
            root_value.as_table_mut().unwrap()
        };

        let systems_entry = root
            .entry("mmorpg_systems".to_string())
            .or_insert_with(|| toml::Value::Table(toml::Table::new()));
        if let Some(systems) = systems_entry.as_table_mut() {
            let mut quests = Vec::new();
            for district in &town.districts {
                let title = format!("Secure {:?}", district.ward);
                let obj = serde_json::json!({
                    "quest_id": format!("q_town_{}", district.id),
                    "title": title,
                    "min_level": 1 + (district.danger * 10.0) as i32,
                    "district_id": district.id,
                    "reward_xp": ((120.0 + district.wealth * 220.0) * self.mmorpg_xp_rate) as i64,
                    "reward_gold": (40.0 + district.wealth * 120.0) as i64
                });
                quests.push(obj);
                added += 1;
            }
            systems.insert(
                "auto_quests_from_town".to_string(),
                toml::Value::String(
                    serde_json::to_string_pretty(&quests).unwrap_or_else(|_| "[]".to_string()),
                ),
            );
        }

        if let Ok(config_text) = toml::to_string_pretty(&root_value) {
            self.project.config = config_text;
        }
        added
    }

    fn spawn_pois_from_last_town(&mut self) -> usize {
        let Some(town) = &self.last_generated_town else {
            return 0;
        };
        let mut count = 0usize;
        if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
            let center = town.size as f32 * 0.5;
            let scale = 0.20f32;
            for landmark in &town.landmarks {
                if let Some(district) = town
                    .districts
                    .iter()
                    .find(|d| d.id == landmark.district_id)
                {
                    let mut entity = rusterix::Entity::new();
                    entity.attributes.set("class_name", Value::Str("npc".to_string()));
                    entity.attributes.set(
                        "display_name",
                        Value::Str(format!("{} Keeper", landmark.name)),
                    );
                    entity.attributes.set("town_generated_poi", Value::Bool(true));
                    entity.position.x = (district.center.0 - center) * scale;
                    entity.position.z = (district.center.1 - center) * scale;
                    map.entities.push(entity);
                    count += 1;
                }
            }
        }
        count
    }

    fn run_mmorpg_sim_tick(&mut self) -> String {
        if let Some(cfg) = &mut self.last_generated_mmorpg {
            let tick = (cfg.world_state.server_tick_ms as f32 / self.mmorpg_event_rate.max(0.1))
                as u64;
            crate::game_logic::tick_world(&mut cfg.world_state, tick.max(1));
            return format!(
                "World tick simulated: +{}ms, world_time={}ms",
                tick.max(1),
                cfg.world_state.world_time_ms
            );
        }
        "No MMORPG config generated yet.".to_string()
    }

    fn run_mmorpg_sim_combat(&mut self) -> String {
        let mut attacker = crate::game_logic::CharacterProfile {
            id: "hero_1".to_string(),
            name: "Hero".to_string(),
            class: crate::game_logic::CharacterClass::Warrior,
            level: 4,
            xp: 0,
            stats: crate::game_logic::StatBlock {
                hp: 180,
                mp: 30,
                attack: (24.0 * self.mmorpg_xp_rate.max(0.1)) as i32,
                defense: 11,
                spell_power: 8,
                crit_chance: 0.18,
                haste: 0.0,
            },
            unlocked_skills: std::collections::BTreeSet::new(),
            equipped_item_level: 8,
        };
        let mut defender = crate::game_logic::CharacterProfile {
            id: "mob_1".to_string(),
            name: "Raider".to_string(),
            class: crate::game_logic::CharacterClass::Rogue,
            level: 3,
            xp: 0,
            stats: crate::game_logic::StatBlock {
                hp: 140,
                mp: 20,
                attack: 18,
                defense: 8,
                spell_power: 2,
                crit_chance: 0.1,
                haste: 0.0,
            },
            unlocked_skills: std::collections::BTreeSet::new(),
            equipped_item_level: 6,
        };
        let ev = crate::game_logic::resolve_combat_event(&attacker, &mut defender, None, 1337);
        let gained = ((80.0 + ev.mitigated_damage as f32 * 2.0) * self.mmorpg_xp_rate.max(0.1))
            as u64;
        let lvups = crate::game_logic::apply_xp(&mut attacker, gained);
        format!(
            "Combat: dmg={}, crit={}, defender_hp={}, attacker_level={}, levelups={}",
            ev.mitigated_damage, ev.did_crit, ev.defender_hp_after, attacker.level, lvups
        )
    }

    fn run_mmorpg_sim_loot(&mut self) -> String {
        let mut entries = vec![
            crate::game_logic::LootTableEntry {
                item_id: "gold_coin".to_string(),
                rarity: crate::game_logic::ItemRarity::Common,
                min_qty: 6,
                max_qty: 22,
                weight: (200.0 * self.mmorpg_loot_rate.max(0.1)) as u32,
            },
            crate::game_logic::LootTableEntry {
                item_id: "healing_potion".to_string(),
                rarity: crate::game_logic::ItemRarity::Uncommon,
                min_qty: 1,
                max_qty: 3,
                weight: (90.0 * self.mmorpg_loot_rate.max(0.1)) as u32,
            },
            crate::game_logic::LootTableEntry {
                item_id: "mystic_shard".to_string(),
                rarity: crate::game_logic::ItemRarity::Rare,
                min_qty: 1,
                max_qty: 1,
                weight: (20.0 * self.mmorpg_loot_rate.max(0.1)) as u32,
            },
        ];
        entries.retain(|e| e.weight > 0);
        let drops = crate::game_logic::roll_loot(&entries, 4242, 3);
        format!("Loot sim generated {} drops", drops.len())
    }
}

impl TheTrait for Editor {

    fn new() -> Self
    where
        Self: Sized,
    {
        println!("[DEBUG] Editor::new() called");
        let mut project = Project::new();
        if let Some(bytes) = crate::Embedded::get("toml/config.toml") {
            if let Ok(source) = std::str::from_utf8(bytes.data.as_ref()) {
                project.config = source.to_string();
            }
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
        let (self_update_tx, self_update_rx) = channel();

        #[cfg(all(
            not(target_arch = "wasm32"),
            feature = "self-update",
            not(target_os = "macos")
        ))]
        let self_updater = SelfUpdater::new("markusmoenig", "Encheament", "encheament-engine");
        #[cfg(all(
            not(target_arch = "wasm32"),
            feature = "self-update",
            target_os = "macos"
        ))]
        let self_updater = SelfUpdater::new("markusmoenig", "Encheament", "Encheament-Engine.app");

        Self {
            project,
            project_path: None,
            sidebar: Sidebar::new(),
            mapeditor: MapEditor::new(),
            server_ctx: ServerContext::default(),
            update_tracker: UpdateTracker::new(),
            event_receiver: None,
            #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
            self_update_rx,
            #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
            self_update_tx,
            #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
            self_updater: Arc::new(Mutex::new(self_updater)),
            update_counter: 0,
            build_values: ValueContainer::default(),
            show_title_page: true,
            show_left_toolbar: true,
            show_right_toolbar: true,
            option_snap_to_grid: true,
            option_show_grid: true,
            option_show_gizmos: true,
            theme_preset: "Dark".to_string(),
            left_toolbar_active_tool: None,
            left_group_modes_expanded: true,
            left_group_2d_expanded: true,
            left_group_3d_expanded: true,
            left_group_editor_expanded: true,
            towngen_preset: "Small City".to_string(),
            towngen_has_river: true,
            towngen_has_walls: true,
            towngen_last_seed: 0,
            towngen_auto_bake: false,
            overlay_show_town_districts: true,
            overlay_show_town_roads: true,
            overlay_show_town_landmarks: true,
            mmorpg_xp_rate: 1.0,
            mmorpg_loot_rate: 1.0,
            mmorpg_event_rate: 1.0,
            mmorpg_world_name: "Encheament Online".to_string(),
            mmorpg_max_players: 600,
            mmorpg_starting_level: 1,
            mmorpg_race_count: 4,
            mmorpg_quest_count: 4,
            mmorpg_skill_tier_count: 2,
            mmorpg_include_warrior: true,
            mmorpg_include_ranger: true,
            mmorpg_include_mage: true,
            mmorpg_include_cleric: true,
            mmorpg_include_rogue: true,
            fantasy_world_name: "Aetheria".to_string(),
            fantasy_world_seed: 0,
            fantasy_continent_count: 3,
            fantasy_countries_per_continent: 7,
            fantasy_towns_per_country: 4,
            fantasy_has_islands: true,
            last_generated_town: None,
            last_generated_mmorpg: None,
            last_generated_fantasy_world: None,
        }
    }

    fn init(&mut self, _ctx: &mut TheContext) {
        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
        {
            let updater = Arc::clone(&self.self_updater);
            let tx = self.self_update_tx.clone();

            thread::spawn(move || {
                let mut updater = updater.lock().unwrap();

                if let Err(err) = updater.fetch_release_list() {
                    tx.send(SelfUpdateEvent::UpdateError(err.to_string()))
                        .unwrap();
                };
            });
        }
    }

    fn window_title(&self) -> String {
        "Encheament Engine".to_string()
    }

    fn target_fps(&self) -> f64 {
        1000.0 / self.redraw_interval_ms() as f64
    }

    fn fonts_to_load(&self) -> Vec<TheFontScript> {
        vec![TheFontScript::Han]
    }

    fn default_window_size(&self) -> (usize, usize) {
        (1200, 720)
    }

    fn window_icon(&self) -> Option<(Vec<u8>, u32, u32)> {
        if let Some(file) = Embedded::get("window_logo.png") {
            let data = std::io::Cursor::new(file.data);

            let decoder = png::Decoder::new(data);
            if let Ok(mut reader) = decoder.read_info() {
                if let Some(buffer_size) = reader.output_buffer_size() {
                    let mut buf = vec![0; buffer_size];
                    let info = reader.next_frame(&mut buf).unwrap();
                    let bytes = &buf[..info.buffer_size()];

                    Some((bytes.to_vec(), info.width, info.height))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn init_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) {
        self.load_workspace_settings_from_project_config();

        // Show dev title page if enabled
        if self.show_title_page {
            let mut vlayout = TheVLayout::new(TheId::named("TitlePageVLayout"));
            vlayout.set_margin(Vec4::new(40, 40, 40, 40));
            vlayout.set_padding(30);
            // ...existing code...
        }

        // Declare all buttons and top_canvas before use
        let mut menubar = TheMenubar::new(TheId::named("Menubar"));
        #[cfg(feature = "staticlib")]
        menubar.limiter_mut().set_max_height(43);
        #[cfg(not(feature = "staticlib"))]
        menubar.limiter_mut().set_max_height(43 + 22);

        let mut logo_button = TheMenubarButton::new(TheId::named("Logo"));
        logo_button.set_icon_name("logo".to_string());
        logo_button.set_status_text(&fl!("status_logo_button"));

        let mut open_button = TheMenubarButton::new(TheId::named("Open"));
        open_button.set_icon_name("icon_role_load".to_string());
        open_button.set_status_text(&fl!("status_open_button"));

        let mut save_button = TheMenubarButton::new(TheId::named("Save"));
        save_button.set_status_text(&fl!("status_save_button"));
        save_button.set_icon_name("icon_role_save".to_string());

        let mut save_as_button = TheMenubarButton::new(TheId::named("Save As"));
        save_as_button.set_icon_name("icon_role_save_as".to_string());
        save_as_button.set_status_text(&fl!("status_save_as_button"));
        save_as_button.set_icon_offset(Vec2::new(2, -5));

        let mut undo_button = TheMenubarButton::new(TheId::named("Undo"));
        undo_button.set_status_text(&fl!("status_undo_button"));
        undo_button.set_icon_name("icon_role_undo".to_string());

        let mut redo_button = TheMenubarButton::new(TheId::named("Redo"));
        redo_button.set_status_text(&fl!("status_redo_button"));
        redo_button.set_icon_name("icon_role_redo".to_string());

        let mut blueprint_button = TheMenubarButton::new(TheId::named("BlueprintEditor"));
        blueprint_button.set_status_text("Open Blueprint Editor");
        blueprint_button.set_icon_name("flowchart".to_string());

        let mut play_button = TheMenubarButton::new(TheId::named("Play"));
        play_button.set_status_text(&fl!("status_play_button"));
        play_button.set_icon_name("play".to_string());

        let mut pause_button = TheMenubarButton::new(TheId::named("Pause"));
        pause_button.set_status_text(&fl!("status_pause_button"));
        pause_button.set_icon_name("play-pause".to_string());

        let mut stop_button = TheMenubarButton::new(TheId::named("Stop"));
        stop_button.set_status_text(&fl!("status_stop_button"));
        stop_button.set_icon_name("stop-fill".to_string());

        let mut input_button = TheMenubarButton::new(TheId::named("GameInput"));
        input_button.set_status_text(&fl!("status_game_input_button"));
        input_button.set_icon_name("keyboard".to_string());
        input_button.set_has_state(true);

        let mut time_slider = TheTimeSlider::new(TheId::named("Server Time Slider"));
        time_slider.set_status_text(&fl!("status_time_slider"));
        time_slider.set_continuous(true);
        time_slider
            .limiter_mut()
            .set_max_width(((ctx.width as i32 * 22) / 100).clamp(140, 320));
        time_slider.set_value(TheValue::Time(TheTime::default()));

        let mut patreon_button = TheMenubarButton::new(TheId::named("Patreon"));
        patreon_button.set_status_text(&fl!("status_patreon_button"));
        patreon_button.set_icon_name("patreon".to_string());
        patreon_button.set_icon_offset(Vec2::new(-4, -2));

        let mut help_button = TheMenubarButton::new(TheId::named("Help"));
        help_button.set_status_text(&fl!("status_help_button"));
        help_button.set_icon_name("question-mark".to_string());
        help_button.set_has_state(true);
        help_button.set_icon_offset(Vec2::new(-2, -2));

        let mut quick_layouts_button = TheMenubarButton::new(TheId::named("TopQuickLayouts"));
        quick_layouts_button.set_status_text("Apply Unreal-like window layout");
        quick_layouts_button.set_icon_name("window".to_string());

        let mut quick_debug_toggle_button =
            TheMenubarButton::new(TheId::named("TopQuickDebugToggle"));
        quick_debug_toggle_button.set_status_text("Toggle runtime (play/stop)");
        quick_debug_toggle_button.set_icon_name("bug".to_string());

        let mut quick_debug_tick_button = TheMenubarButton::new(TheId::named("TopQuickDebugTick"));
        quick_debug_tick_button.set_status_text("Run one debug simulation tick");
        quick_debug_tick_button.set_icon_name("activity".to_string());

        let mut quick_source_submit_button =
            TheMenubarButton::new(TheId::named("TopQuickSourceSubmit"));
        quick_source_submit_button.set_status_text("Submit source control content");
        quick_source_submit_button.set_icon_name("upload".to_string());

        let mut quick_source_sync_button =
            TheMenubarButton::new(TheId::named("TopQuickSourceSync"));
        quick_source_sync_button.set_status_text("Sync source control");
        quick_source_sync_button.set_icon_name("arrow-clockwise".to_string());

        let mut quick_mode_select_button = TheMenubarButton::new(TheId::named("TopQuickModeSelect"));
        quick_mode_select_button.set_status_text("Switch to Select mode");
        quick_mode_select_button.set_icon_name("cursor".to_string());

        let mut quick_mode_landscape_button =
            TheMenubarButton::new(TheId::named("TopQuickModeLandscape"));
        quick_mode_landscape_button.set_status_text("Switch to Landscape mode");
        quick_mode_landscape_button.set_icon_name("map".to_string());

        let mut quick_platform_windows_button =
            TheMenubarButton::new(TheId::named("TopQuickPlatformWindows"));
        quick_platform_windows_button.set_status_text("Package project for Windows");
        quick_platform_windows_button.set_icon_name("box".to_string());

        let mut quick_platform_web_button =
            TheMenubarButton::new(TheId::named("TopQuickPlatformWeb"));
        quick_platform_web_button.set_status_text("Package project for Web");
        quick_platform_web_button.set_icon_name("globe".to_string());

        let mut quick_mapforge_open_button =
            TheMenubarButton::new(TheId::named("TopQuickMapForgeOpen"));
        quick_mapforge_open_button.set_status_text("Open Map Forge editor");
        quick_mapforge_open_button.set_icon_name("compass".to_string());

        let mut quick_mapforge_generate_button =
            TheMenubarButton::new(TheId::named("TopQuickMapForgeGenerateTown"));
        quick_mapforge_generate_button.set_status_text("Generate Map Forge town");
        quick_mapforge_generate_button.set_icon_name("sparkle".to_string());

        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
        let mut update_button = {
            let mut button = TheMenubarButton::new(TheId::named("Update"));
            button.set_status_text(&fl!("status_update_button"));
            button.set_icon_name("arrows-clockwise".to_string());
            button
        };

        let mut hlayout = TheHLayout::new(TheId::named("Menu Layout"));
        hlayout.set_background_color(None);
        hlayout.set_margin(Vec4::new(10, 2, 10, 1));
        hlayout.add_widget(Box::new(logo_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(open_button));
        hlayout.add_widget(Box::new(save_button));
        hlayout.add_widget(Box::new(save_as_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(undo_button));
        hlayout.add_widget(Box::new(redo_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(blueprint_button));
        hlayout.add_widget(Box::new(play_button));
        hlayout.add_widget(Box::new(pause_button));
        hlayout.add_widget(Box::new(stop_button));
        hlayout.add_widget(Box::new(input_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(quick_layouts_button));
        hlayout.add_widget(Box::new(quick_debug_toggle_button));
        hlayout.add_widget(Box::new(quick_debug_tick_button));
        hlayout.add_widget(Box::new(quick_source_submit_button));
        hlayout.add_widget(Box::new(quick_source_sync_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(quick_mode_select_button));
        hlayout.add_widget(Box::new(quick_mode_landscape_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(quick_platform_windows_button));
        hlayout.add_widget(Box::new(quick_platform_web_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(quick_mapforge_open_button));
        hlayout.add_widget(Box::new(quick_mapforge_generate_button));
        hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
        hlayout.add_widget(Box::new(time_slider));

        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
        {
            hlayout.add_widget(Box::new(update_button));
            hlayout.add_widget(Box::new(TheMenubarSeparator::new(TheId::empty())));
            hlayout.add_widget(Box::new(patreon_button));
            hlayout.set_reverse_index(Some(3));
        }

        #[cfg(not(all(not(target_arch = "wasm32"), feature = "self-update")))]
        {
            hlayout.add_widget(Box::new(patreon_button));
            hlayout.add_widget(Box::new(help_button));
            hlayout.set_reverse_index(Some(2));
        }

        let mut top_canvas = TheCanvas::new();
        top_canvas.set_widget(menubar);
        top_canvas.set_layout(hlayout);

        // Main top menus + submenus
        let mut menu_canvas = TheCanvas::new();
        let mut main_menu = TheMenu::new(TheId::named("Main Menu"));

        let mut file_menu = TheContextMenu::named("File".to_string());
        file_menu.add(TheContextMenuItem::new("New".to_string(), TheId::named("MenuFile::New")));
        file_menu.add(TheContextMenuItem::new("Open...".to_string(), TheId::named("MenuFile::Open")));
        file_menu.add(TheContextMenuItem::new("Save".to_string(), TheId::named("MenuFile::Save")));
        file_menu.add(TheContextMenuItem::new("Save As...".to_string(), TheId::named("MenuFile::SaveAs")));
        file_menu.add_separator();
        file_menu.add(TheContextMenuItem::new("Close Project".to_string(), TheId::named("MenuFile::Close")));

        let mut edit_menu = TheContextMenu::named("Edit".to_string());
        edit_menu.add(TheContextMenuItem::new("Undo".to_string(), TheId::named("MenuEdit::Undo")));
        edit_menu.add(TheContextMenuItem::new("Redo".to_string(), TheId::named("MenuEdit::Redo")));
        edit_menu.add_separator();
        edit_menu.add(TheContextMenuItem::new("Copy".to_string(), TheId::named("MenuEdit::Copy")));
        edit_menu.add(TheContextMenuItem::new("Paste".to_string(), TheId::named("MenuEdit::Paste")));

        let mut view_menu = TheContextMenu::named("View".to_string());
        view_menu.add(TheContextMenuItem::new("Toggle Left Toolbar".to_string(), TheId::named("MenuView::ToggleLeft")));
        view_menu.add(TheContextMenuItem::new("Toggle Right Toolbar".to_string(), TheId::named("MenuView::ToggleRight")));
        view_menu.add_separator();
        view_menu.add(TheContextMenuItem::new("Theme: Dark".to_string(), TheId::named("MenuTheme::Dark")));
        view_menu.add(TheContextMenuItem::new("Theme: Light".to_string(), TheId::named("MenuTheme::Light")));
        view_menu.add(TheContextMenuItem::new("Theme: Slate".to_string(), TheId::named("MenuTheme::Slate")));

        let mut select_filter_menu = TheContextMenu::named("Select Filter".to_string());
        select_filter_menu.add(TheContextMenuItem::new(
            "Geometry".to_string(),
            TheId::named("MenuSelect::FilterGeometry"),
        ));
        select_filter_menu.add(TheContextMenuItem::new(
            "Entities".to_string(),
            TheId::named("MenuSelect::FilterEntity"),
        ));
        select_filter_menu.add(TheContextMenuItem::new(
            "Landmarks".to_string(),
            TheId::named("MenuSelect::FilterLandmark"),
        ));
        let mut select_menu = TheContextMenu::named("Select".to_string());
        select_menu.add(TheContextMenuItem::new(
            "Select All".to_string(),
            TheId::named("MenuSelect::All"),
        ));
        select_menu.add(TheContextMenuItem::new(
            "Select None".to_string(),
            TheId::named("MenuSelect::None"),
        ));
        select_menu.add(TheContextMenuItem::new(
            "Invert Selection".to_string(),
            TheId::named("MenuSelect::Invert"),
        ));
        select_menu.add(TheContextMenuItem::new_submenu(
            "Filter".to_string(),
            TheId::named("MenuSelect::Filter"),
            select_filter_menu,
        ));

        let mut actor_place_menu = TheContextMenu::named("Actor Placement".to_string());
        actor_place_menu.add(TheContextMenuItem::new(
            "Entity".to_string(),
            TheId::named("MenuActor::PlaceEntity"),
        ));
        actor_place_menu.add(TheContextMenuItem::new(
            "Landmark".to_string(),
            TheId::named("MenuActor::PlaceLandmark"),
        ));
        actor_place_menu.add(TheContextMenuItem::new(
            "Terrain Brush".to_string(),
            TheId::named("MenuActor::PlaceTerrain"),
        ));
        let mut actor_menu = TheContextMenu::named("Actor".to_string());
        actor_menu.add(TheContextMenuItem::new_submenu(
            "Place Actor".to_string(),
            TheId::named("MenuActor::Place"),
            actor_place_menu,
        ));
        actor_menu.add(TheContextMenuItem::new(
            "Align To Grid".to_string(),
            TheId::named("MenuActor::AlignToGrid"),
        ));
        actor_menu.add(TheContextMenuItem::new(
            "Snap To Grid".to_string(),
            TheId::named("MenuActor::SnapToGrid"),
        ));
        actor_menu.add(TheContextMenuItem::new(
            "Focus Selection".to_string(),
            TheId::named("MenuActor::FocusSelection"),
        ));

        let mut blueprint_create_menu = TheContextMenu::named("Create Blueprint".to_string());
        blueprint_create_menu.add(TheContextMenuItem::new(
            "Character Blueprint".to_string(),
            TheId::named("MenuBlueprint::CreateCharacter"),
        ));
        blueprint_create_menu.add(TheContextMenuItem::new(
            "Item Blueprint".to_string(),
            TheId::named("MenuBlueprint::CreateItem"),
        ));
        blueprint_create_menu.add(TheContextMenuItem::new(
            "Quest Blueprint".to_string(),
            TheId::named("MenuBlueprint::CreateQuest"),
        ));
        let mut blueprint_menu = TheContextMenu::named("Blueprint".to_string());
        blueprint_menu.add(TheContextMenuItem::new(
            "Open Blueprint Panel".to_string(),
            TheId::named("MenuBlueprint::OpenPanel"),
        ));
        blueprint_menu.add(TheContextMenuItem::new_submenu(
            "Create".to_string(),
            TheId::named("MenuBlueprint::Create"),
            blueprint_create_menu,
        ));
        blueprint_menu.add(TheContextMenuItem::new(
            "Compile All Blueprints".to_string(),
            TheId::named("MenuBlueprint::CompileAll"),
        ));
        blueprint_menu.add(TheContextMenuItem::new(
            "Validate Blueprint Links".to_string(),
            TheId::named("MenuBlueprint::Validate"),
        ));

        let mut cinematics_menu = TheContextMenu::named("Cinematics".to_string());
        cinematics_menu.add(TheContextMenuItem::new(
            "Create Level Sequence".to_string(),
            TheId::named("MenuCinematics::CreateSequence"),
        ));
        cinematics_menu.add(TheContextMenuItem::new(
            "Add Camera Track".to_string(),
            TheId::named("MenuCinematics::AddCameraTrack"),
        ));
        cinematics_menu.add(TheContextMenuItem::new(
            "Play Sequence".to_string(),
            TheId::named("MenuCinematics::PlaySequence"),
        ));

        let mut modes_menu = TheContextMenu::named("Modes".to_string());
        modes_menu.add(TheContextMenuItem::new(
            "Select Mode".to_string(),
            TheId::named("MenuMode::Select"),
        ));
        modes_menu.add(TheContextMenuItem::new(
            "Landscape Mode".to_string(),
            TheId::named("MenuMode::Landscape"),
        ));
        modes_menu.add(TheContextMenuItem::new(
            "Modeling Mode".to_string(),
            TheId::named("MenuMode::Modeling"),
        ));
        modes_menu.add(TheContextMenuItem::new(
            "Foliage Mode".to_string(),
            TheId::named("MenuMode::Foliage"),
        ));
        modes_menu.add(TheContextMenuItem::new(
            "Brush Editing Mode".to_string(),
            TheId::named("MenuMode::BrushEditing"),
        ));
        modes_menu.add(TheContextMenuItem::new(
            "Animation Mode".to_string(),
            TheId::named("MenuMode::Animation"),
        ));
        modes_menu.add_separator();
        modes_menu.add(TheContextMenuItem::new(
            "Game View".to_string(),
            TheId::named("MenuMode::GameView"),
        ));

        let mut platforms_package_menu = TheContextMenu::named("Package Project".to_string());
        platforms_package_menu.add(TheContextMenuItem::new(
            "Windows".to_string(),
            TheId::named("MenuPlatforms::PackageWindows"),
        ));
        platforms_package_menu.add(TheContextMenuItem::new(
            "Linux".to_string(),
            TheId::named("MenuPlatforms::PackageLinux"),
        ));
        platforms_package_menu.add(TheContextMenuItem::new(
            "macOS".to_string(),
            TheId::named("MenuPlatforms::PackageMac"),
        ));
        platforms_package_menu.add(TheContextMenuItem::new(
            "Web".to_string(),
            TheId::named("MenuPlatforms::PackageWeb"),
        ));

        let mut platforms_menu = TheContextMenu::named("Platforms".to_string());
        platforms_menu.add(TheContextMenuItem::new_submenu(
            "Package Project".to_string(),
            TheId::named("MenuPlatforms::Package"),
            platforms_package_menu,
        ));
        platforms_menu.add(TheContextMenuItem::new(
            "Project Settings".to_string(),
            TheId::named("MenuPlatforms::ProjectSettings"),
        ));
        platforms_menu.add(TheContextMenuItem::new(
            "Device Manager".to_string(),
            TheId::named("MenuPlatforms::DeviceManager"),
        ));
        platforms_menu.add(TheContextMenuItem::new(
            "SDK Manager".to_string(),
            TheId::named("MenuPlatforms::SdkManager"),
        ));

        let mut layouts_menu = TheContextMenu::named("Window Layouts".to_string());
        layouts_menu.add(TheContextMenuItem::new(
            "Unreal-like Layout".to_string(),
            TheId::named("MenuLayouts::Unreal"),
        ));
        layouts_menu.add(TheContextMenuItem::new(
            "Minimal Layout".to_string(),
            TheId::named("MenuLayouts::Minimal"),
        ));
        layouts_menu.add(TheContextMenuItem::new(
            "Feature Matrix".to_string(),
            TheId::named("MenuLayouts::FeatureMatrix"),
        ));

        let mut debug_menu = TheContextMenu::named("Debug".to_string());
        debug_menu.add(TheContextMenuItem::new(
            "Toggle Runtime".to_string(),
            TheId::named("MenuDebug::ToggleRuntime"),
        ));
        debug_menu.add(TheContextMenuItem::new(
            "Run Sim Tick".to_string(),
            TheId::named("MenuDebug::SimTick"),
        ));
        debug_menu.add(TheContextMenuItem::new(
            "Run Sim Combat".to_string(),
            TheId::named("MenuDebug::SimCombat"),
        ));
        debug_menu.add(TheContextMenuItem::new(
            "Run Sim Loot".to_string(),
            TheId::named("MenuDebug::SimLoot"),
        ));
        debug_menu.add(TheContextMenuItem::new(
            "Show Feature Matrix".to_string(),
            TheId::named("MenuDebug::FeatureMatrix"),
        ));

        let mut source_control_menu = TheContextMenu::named("Source Control".to_string());
        source_control_menu.add(TheContextMenuItem::new(
            "Connect".to_string(),
            TheId::named("MenuSource::Connect"),
        ));
        source_control_menu.add(TheContextMenuItem::new(
            "Submit Content".to_string(),
            TheId::named("MenuSource::SubmitContent"),
        ));
        source_control_menu.add(TheContextMenuItem::new(
            "Sync".to_string(),
            TheId::named("MenuSource::Sync"),
        ));
        source_control_menu.add(TheContextMenuItem::new(
            "Revert Unchanged".to_string(),
            TheId::named("MenuSource::RevertUnchanged"),
        ));

        let mut tools_2d = TheContextMenu::named("2D".to_string());
        tools_2d.add(TheContextMenuItem::new("Selection".to_string(), TheId::named("MenuTool::Select Tool")));
        tools_2d.add(TheContextMenuItem::new("Vertex".to_string(), TheId::named("MenuTool::Vertex Tool")));
        tools_2d.add(TheContextMenuItem::new("Linedef".to_string(), TheId::named("MenuTool::Linedef Tool")));
        tools_2d.add(TheContextMenuItem::new("Sector".to_string(), TheId::named("MenuTool::Sector Tool")));
        tools_2d.add(TheContextMenuItem::new("Rect".to_string(), TheId::named("MenuTool::Rect Tool")));

        let mut tools_3d = TheContextMenu::named("3D".to_string());
        tools_3d.add(TheContextMenuItem::new("Terrain".to_string(), TheId::named("MenuTool::World Tool")));
        tools_3d.add(TheContextMenuItem::new("Render".to_string(), TheId::named("MenuTool::Render Tool")));
        tools_3d.add(TheContextMenuItem::new("Entity".to_string(), TheId::named("MenuTool::Entity Tool")));
        tools_3d.add(TheContextMenuItem::new("Game Runtime".to_string(), TheId::named("MenuTool::Game Tool")));

        let mut tools_editor = TheContextMenu::named("Editor".to_string());
        tools_editor.add(TheContextMenuItem::new("Code".to_string(), TheId::named("MenuTool::Code Tool")));
        tools_editor.add(TheContextMenuItem::new("Data".to_string(), TheId::named("MenuTool::Data Tool")));
        tools_editor.add(TheContextMenuItem::new("Tileset".to_string(), TheId::named("MenuTool::Tileset Tool")));
        tools_editor.add(TheContextMenuItem::new("Config".to_string(), TheId::named("MenuTool::Config Tool")));
        tools_editor.add(TheContextMenuItem::new("Info".to_string(), TheId::named("MenuTool::Info Tool")));

        let mut all_tools_menu = TheContextMenu::named("All Tools".to_string());
        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let tool_name = tool.id().name.clone();
                let tool_label = tool
                    .id()
                    .name
                    .strip_suffix(" Tool")
                    .unwrap_or(&tool.id().name)
                    .to_string();
                all_tools_menu.add(TheContextMenuItem::new(
                    tool_label,
                    TheId::named(&format!("MenuTool::{}", tool_name)),
                ));
            }
        }

        let mut tools_menu = TheContextMenu::named("Tools".to_string());
        tools_menu.add(TheContextMenuItem::new_submenu(
            "2D Tools".to_string(),
            TheId::named("MenuTools::2D"),
            tools_2d,
        ));
        tools_menu.add(TheContextMenuItem::new_submenu(
            "3D Tools".to_string(),
            TheId::named("MenuTools::3D"),
            tools_3d,
        ));
        tools_menu.add(TheContextMenuItem::new_submenu(
            "Editor Tools".to_string(),
            TheId::named("MenuTools::Editor"),
            tools_editor,
        ));
        tools_menu.add(TheContextMenuItem::new_submenu(
            "All Tools".to_string(),
            TheId::named("MenuTools::All"),
            all_tools_menu,
        ));
        tools_menu.add_separator();
        tools_menu.add(TheContextMenuItem::new(
            "Generate Town (Watabou-style)".to_string(),
            TheId::named("MenuTools::GenerateTown"),
        ));
        tools_menu.add(TheContextMenuItem::new(
            "Generate Fantasy World (Continent/Country)".to_string(),
            TheId::named("MenuTools::GenerateFantasyWorld"),
        ));
        let mut town_preset_menu = TheContextMenu::named("Town Presets".to_string());
        town_preset_menu.add(TheContextMenuItem::new(
            "Small Town".to_string(),
            TheId::named("MenuTownPreset::SmallTown"),
        ));
        town_preset_menu.add(TheContextMenuItem::new(
            "Large Town".to_string(),
            TheId::named("MenuTownPreset::LargeTown"),
        ));
        town_preset_menu.add(TheContextMenuItem::new(
            "Small City".to_string(),
            TheId::named("MenuTownPreset::SmallCity"),
        ));
        town_preset_menu.add(TheContextMenuItem::new(
            "Large City".to_string(),
            TheId::named("MenuTownPreset::LargeCity"),
        ));

        let mut town_ops_menu = TheContextMenu::named("Town Ops".to_string());
        town_ops_menu.add(TheContextMenuItem::new_submenu(
            "Town Size Presets".to_string(),
            TheId::named("MenuTools::TownPresets"),
            town_preset_menu,
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Reseed + Generate".to_string(),
            TheId::named("MenuTown::ReseedGenerate"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Regenerate Same Seed".to_string(),
            TheId::named("MenuTown::Regenerate"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Toggle River".to_string(),
            TheId::named("MenuTown::ToggleRiver"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Toggle Walls".to_string(),
            TheId::named("MenuTown::ToggleWalls"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Bake To Current Map".to_string(),
            TheId::named("MenuTown::BakeMap"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Export JSON".to_string(),
            TheId::named("MenuTown::ExportJson"),
        ));
        town_ops_menu.add(TheContextMenuItem::new(
            "Town: Import JSON".to_string(),
            TheId::named("MenuTown::ImportJson"),
        ));
        tools_menu.add(TheContextMenuItem::new_submenu(
            "Town Systems".to_string(),
            TheId::named("MenuTools::TownSystems"),
            town_ops_menu,
        ));

        let mut mmo_ops_menu = TheContextMenu::named("MMO Ops".to_string());
        mmo_ops_menu.add(TheContextMenuItem::new(
            "Town: Generate MMO Quests".to_string(),
            TheId::named("MenuTown::AutoQuest"),
        ));
        mmo_ops_menu.add(TheContextMenuItem::new(
            "Town: Spawn MMO POIs".to_string(),
            TheId::named("MenuTown::SpawnPoi"),
        ));
        mmo_ops_menu.add(TheContextMenuItem::new(
            "Generate RPG/MMORPG Systems".to_string(),
            TheId::named("MenuTools::GenerateRpgMmorpg"),
        ));
        let mut mmorpg_builder_menu = TheContextMenu::named("RPG/MMORPG Builder".to_string());
        mmorpg_builder_menu.add(TheContextMenuItem::new(
            "Open Builder Summary".to_string(),
            TheId::named("MenuMmorpgBuilder::Open"),
        ));
        mmorpg_builder_menu.add(TheContextMenuItem::new(
            "Generate from Builder Inputs".to_string(),
            TheId::named("MenuMmorpgBuilder::Generate"),
        ));
        mmo_ops_menu.add(TheContextMenuItem::new_submenu(
            "Builder".to_string(),
            TheId::named("MenuMmorpgBuilder::Submenu"),
            mmorpg_builder_menu,
        ));
        mmo_ops_menu.add(TheContextMenuItem::new(
            "MMO Sim: Tick".to_string(),
            TheId::named("MenuMmoSim::Tick"),
        ));
        mmo_ops_menu.add(TheContextMenuItem::new(
            "MMO Sim: Combat".to_string(),
            TheId::named("MenuMmoSim::Combat"),
        ));
        mmo_ops_menu.add(TheContextMenuItem::new(
            "MMO Sim: Loot".to_string(),
            TheId::named("MenuMmoSim::Loot"),
        ));
        tools_menu.add(TheContextMenuItem::new_submenu(
            "RPG/MMORPG".to_string(),
            TheId::named("MenuTools::RpgMmorpg"),
            mmo_ops_menu,
        ));

        let mut map_forge_tools_menu = TheContextMenu::named("Map Forge Tools".to_string());
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Open Map Editor".to_string(),
            TheId::named("MenuMapForge::OpenEditor"),
        ));
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Tool: Select".to_string(),
            TheId::named("MenuMapForge::ToolSelect"),
        ));
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Tool: Terrain".to_string(),
            TheId::named("MenuMapForge::ToolTerrain"),
        ));
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Tool: Roads".to_string(),
            TheId::named("MenuMapForge::ToolRoad"),
        ));
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Tool: Districts".to_string(),
            TheId::named("MenuMapForge::ToolDistrict"),
        ));
        map_forge_tools_menu.add(TheContextMenuItem::new(
            "Tool: Landmarks".to_string(),
            TheId::named("MenuMapForge::ToolLandmark"),
        ));

        let mut map_forge_layers_menu = TheContextMenu::named("Map Forge Layers".to_string());
        map_forge_layers_menu.add(TheContextMenuItem::new(
            "Toggle Districts".to_string(),
            TheId::named("MenuMapForge::LayerDistricts"),
        ));
        map_forge_layers_menu.add(TheContextMenuItem::new(
            "Toggle Roads".to_string(),
            TheId::named("MenuMapForge::LayerRoads"),
        ));
        map_forge_layers_menu.add(TheContextMenuItem::new(
            "Toggle Landmarks".to_string(),
            TheId::named("MenuMapForge::LayerLandmarks"),
        ));

        let mut map_forge_generate_menu = TheContextMenu::named("Map Forge Generate".to_string());
        map_forge_generate_menu.add(TheContextMenuItem::new(
            "Generate Town".to_string(),
            TheId::named("MenuMapForge::GenerateTown"),
        ));
        map_forge_generate_menu.add(TheContextMenuItem::new(
            "Generate Fantasy World".to_string(),
            TheId::named("MenuMapForge::GenerateFantasyWorld"),
        ));
        map_forge_generate_menu.add(TheContextMenuItem::new(
            "Reseed + Generate Town".to_string(),
            TheId::named("MenuMapForge::ReseedTown"),
        ));
        map_forge_generate_menu.add(TheContextMenuItem::new(
            "Regenerate Town".to_string(),
            TheId::named("MenuMapForge::RegenerateTown"),
        ));

        let mut map_forge_io_menu = TheContextMenu::named("Map Forge IO".to_string());
        map_forge_io_menu.add(TheContextMenuItem::new(
            "Export JSON".to_string(),
            TheId::named("MenuMapForge::ExportJson"),
        ));
        map_forge_io_menu.add(TheContextMenuItem::new(
            "Import JSON".to_string(),
            TheId::named("MenuMapForge::ImportJson"),
        ));
        map_forge_io_menu.add(TheContextMenuItem::new(
            "Bake To Current Map".to_string(),
            TheId::named("MenuMapForge::BakeMap"),
        ));

        let mut map_forge_systems_menu = TheContextMenu::named("Map Forge Systems".to_string());
        map_forge_systems_menu.add(TheContextMenuItem::new_submenu(
            "Editor".to_string(),
            TheId::named("MenuMapForge::Editor"),
            map_forge_tools_menu,
        ));
        map_forge_systems_menu.add(TheContextMenuItem::new_submenu(
            "Layers".to_string(),
            TheId::named("MenuMapForge::Layers"),
            map_forge_layers_menu,
        ));
        map_forge_systems_menu.add(TheContextMenuItem::new_submenu(
            "Generate".to_string(),
            TheId::named("MenuMapForge::Generate"),
            map_forge_generate_menu,
        ));
        map_forge_systems_menu.add(TheContextMenuItem::new_submenu(
            "Import / Export".to_string(),
            TheId::named("MenuMapForge::IO"),
            map_forge_io_menu,
        ));

        tools_menu.add(TheContextMenuItem::new_submenu(
            "Map Forge Systems".to_string(),
            TheId::named("MenuTools::MapForgeSystems"),
            map_forge_systems_menu,
        ));

        let mut build_menu = TheContextMenu::named("Build".to_string());
        build_menu.add(TheContextMenuItem::new("Play".to_string(), TheId::named("MenuBuild::Play")));
        build_menu.add(TheContextMenuItem::new("Pause".to_string(), TheId::named("MenuBuild::Pause")));
        build_menu.add(TheContextMenuItem::new("Stop".to_string(), TheId::named("MenuBuild::Stop")));
        build_menu.add_separator();
        build_menu.add(TheContextMenuItem::new("Build 2D Export".to_string(), TheId::named("MenuBuild::Export2D")));
        build_menu.add(TheContextMenuItem::new("Build 3D Export".to_string(), TheId::named("MenuBuild::Export3D")));

        let mut settings_menu = TheContextMenu::named("Settings".to_string());
        settings_menu.add(TheContextMenuItem::new("Toggle Snap".to_string(), TheId::named("MenuOption::Snap")));
        settings_menu.add(TheContextMenuItem::new("Toggle Grid".to_string(), TheId::named("MenuOption::Grid")));
        settings_menu.add(TheContextMenuItem::new("Toggle Gizmos".to_string(), TheId::named("MenuOption::Gizmos")));
        settings_menu.add_separator();
        let mut ide_layout_menu = TheContextMenu::named("IDE Layout Presets".to_string());
        ide_layout_menu.add(TheContextMenuItem::new(
            "Unreal-like Layout".to_string(),
            TheId::named("MenuIde::LayoutUnreal"),
        ));
        ide_layout_menu.add(TheContextMenuItem::new(
            "Minimal Layout".to_string(),
            TheId::named("MenuIde::LayoutMinimal"),
        ));
        settings_menu.add(TheContextMenuItem::new_submenu(
            "IDE Layout".to_string(),
            TheId::named("MenuIde::LayoutSubmenu"),
            ide_layout_menu,
        ));
        settings_menu.add(TheContextMenuItem::new(
            "IDE Feature Matrix".to_string(),
            TheId::named("MenuIde::FeatureMatrix"),
        ));

        let mut window_menu = TheContextMenu::named("Window".to_string());
        window_menu.add(TheContextMenuItem::new(
            "Content Browser".to_string(),
            TheId::named("MenuWindow::ContentBrowser"),
        ));
        window_menu.add(TheContextMenuItem::new(
            "World Outliner".to_string(),
            TheId::named("MenuWindow::WorldOutliner"),
        ));
        window_menu.add(TheContextMenuItem::new(
            "Details".to_string(),
            TheId::named("MenuWindow::Details"),
        ));
        window_menu.add(TheContextMenuItem::new(
            "Output Log".to_string(),
            TheId::named("MenuWindow::OutputLog"),
        ));
        window_menu.add(TheContextMenuItem::new(
            "Blueprint".to_string(),
            TheId::named("MenuWindow::Blueprint"),
        ));

        let mut help_menu = TheContextMenu::named("Help".to_string());
        help_menu.add(TheContextMenuItem::new("Docs".to_string(), TheId::named("MenuHelp::Docs")));
        help_menu.add(TheContextMenuItem::new("Examples".to_string(), TheId::named("MenuHelp::Examples")));
        help_menu.add(TheContextMenuItem::new("About".to_string(), TheId::named("MenuHelp::About")));

        main_menu.add_context_menu(file_menu);
        main_menu.add_context_menu(edit_menu);
        main_menu.add_context_menu(view_menu);
        main_menu.add_context_menu(select_menu);
        main_menu.add_context_menu(actor_menu);
        main_menu.add_context_menu(blueprint_menu);
        main_menu.add_context_menu(cinematics_menu);
        main_menu.add_context_menu(modes_menu);
        main_menu.add_context_menu(platforms_menu);
        main_menu.add_context_menu(layouts_menu);
        main_menu.add_context_menu(debug_menu);
        main_menu.add_context_menu(source_control_menu);
        main_menu.add_context_menu(tools_menu);
        main_menu.add_context_menu(build_menu);
        main_menu.add_context_menu(settings_menu);
        main_menu.add_context_menu(window_menu);
        main_menu.add_context_menu(help_menu);

        menu_canvas.set_widget(main_menu);
        top_canvas.set_top(menu_canvas);

        // Top quick tool bar
        let mut top_quick_canvas = TheCanvas::new();
        top_quick_canvas.set_widget(TheToolListBar::new(TheId::named("Top Quick Tool Bar")));
        let mut top_quick_layout = TheHLayout::new(TheId::named("Top Quick Tool Layout"));
        let compact_top = self.is_compact_ui(ctx);
        top_quick_layout.set_background_color(None);
        if compact_top {
            top_quick_layout.set_margin(Vec4::new(6, 1, 6, 1));
            top_quick_layout.set_padding(0);
        } else {
            top_quick_layout.set_margin(Vec4::new(10, 2, 10, 2));
            top_quick_layout.set_padding(1);
        }
        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let quick_id = format!("TopTool::{}", tool.id().name);
                let mut button = TheTraybarButton::new(TheId::named(&quick_id));
                button.set_icon_name(tool.icon_name());
                let tool_name = tool
                    .id()
                    .name
                    .strip_suffix(" Tool")
                    .unwrap_or(&tool.id().name)
                    .to_string();
                if compact_top {
                    button.set_text(String::new());
                } else {
                    button.set_text(tool_name);
                }
                button.set_status_text(&format!("Activate {}", tool.info()));
                top_quick_layout.add_widget(Box::new(button));
            }
        }
        top_quick_canvas.set_layout(top_quick_layout);
        top_canvas.set_bottom(top_quick_canvas);
        ui.canvas.set_top(top_canvas);

        let workspace_canvas = self.build_ue5_workspace_canvas(ui, ctx);
        ui.canvas.set_center(workspace_canvas);
        ui.canvas.set_bottom(self.build_status_canvas());

        self.apply_toolbar_visibility(ui, ctx);
        self.apply_theme_preset(ui, ctx);
        self.sync_left_toolbar_active_from_toollist(ctx);
        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
        self.server_ctx.show_editing_geometry = self.option_show_gizmos;
        self.persist_workspace_settings_to_project_config();

        // -

        // ctx.ui.set_disabled("Save");
        // ctx.ui.set_disabled("Save As");
        ctx.ui.set_disabled("Undo");
        ctx.ui.set_disabled("Redo");

        // Init Rusterix

        if let Some(icon) = ctx.ui.icon("light_on") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values.set("light_on", Value::Texture(texture));
        }
        if let Some(icon) = ctx.ui.icon("light_off") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values.set("light_off", Value::Texture(texture));
        }
        if let Some(icon) = ctx.ui.icon("character_on") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values
                .set("character_on", Value::Texture(texture));
        }
        if let Some(icon) = ctx.ui.icon("character_off") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values
                .set("character_off", Value::Texture(texture));
        }
        if let Some(icon) = ctx.ui.icon("treasure_on") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values
                .set("treasure_on", Value::Texture(texture));
        }
        if let Some(icon) = ctx.ui.icon("treasure_off") {
            let texture = Texture::from_rgbabuffer(icon);
            self.build_values
                .set("treasure_off", Value::Texture(texture));
        }

        RUSTERIX
            .write()
            .unwrap()
            .client
            .builder_d2
            .set_properties(&self.build_values);
        RUSTERIX.write().unwrap().set_d2();
        SCENEMANAGER.write().unwrap().startup();

        self.event_receiver = Some(ui.add_state_listener("Main Receiver".into()));
    }

    /// Set the command line arguments
    fn set_cmd_line_args(&mut self, args: Vec<String>, ctx: &mut TheContext) {
        if args.len() > 1 {
            #[allow(irrefutable_let_patterns)]
            if let Ok(path) = PathBuf::from_str(&args[1]) {
                ctx.ui.send(TheEvent::FileRequesterResult(
                    TheId::named("Open"),
                    vec![path],
                ));
                return;
            }
        }

        ctx.ui.send(TheEvent::StateChanged(
            TheId::named("New"),
            TheWidgetState::Clicked,
        ));
    }

    /// Handle UI events and UI state
    fn update_ui(&mut self, ui: &mut TheUI, ctx: &mut TheContext) -> bool {
        let mut redraw = false;
        let mut update_server_icons = false;
        let mut workspace_prefs_dirty = false;
        self.sync_left_toolbar_active_from_toollist(ctx);

        // Make sure on first startup the active tool is properly selected
        if self.update_counter == 0 {
            let mut toollist = TOOLLIST.write().unwrap();
            let id = toollist.get_current_tool().id().uuid;

            toollist.set_tool(id, ui, ctx, &mut self.project, &mut self.server_ctx);
        }

        // Get build results from the scene manager if any
        while let Some(result) = SCENEMANAGER.write().unwrap().receive() {
            match result {
                SceneManagerResult::Startup => {
                    println!("Scene manager has started up.");
                }
                SceneManagerResult::ProcessedHeights(coord, heights) => {
                    if let Some(map) = &mut self.project.get_map_mut(&self.server_ctx) {
                        let local = map.terrain.get_chunk_coords(coord.x, coord.y);
                        if let Some(chunk) = &mut map.terrain.chunks.get_mut(&local) {
                            chunk.processed_heights = Some(heights);
                        }
                    }
                }
                SceneManagerResult::Chunk(chunk, togo, total, _billboards) => {
                    let (_chunk, _total) = (chunk, total);
                    if togo == 0 {
                        self.server_ctx.background_progress = None;
                    }
                }
                SceneManagerResult::UpdatedBatch3D(coord, batch) => {
                    let mut rusterix = RUSTERIX.write().unwrap();
                    if let Some(chunk) = rusterix.client.scene.chunks.get_mut(&coord) {
                        chunk.terrain_batch3d = Some(batch);
                    }
                }
                SceneManagerResult::Clear => {
                    let mut rusterix = RUSTERIX.write().unwrap();
                    rusterix
                        .scene_handler
                        .vm
                        .execute(scenevm::Atom::ClearGeometry);

                    rusterix.scene_handler.billboards.clear();
                }
                SceneManagerResult::Quit => {
                    println!("Scene manager has shutdown.");
                }
            }
        }

        // Check for redraw (30fps) and tick updates
        let redraw_ms = self.redraw_interval_ms();
        let tick_ms = CONFIGEDITOR.read().unwrap().game_tick_ms.max(1) as u64;
        let (mut redraw_update, tick_update) = self.update_tracker.update(redraw_ms, tick_ms);

        // Handle queued UI events in the same update pass so input can trigger immediate redraw work.
        let mut pending_events = Vec::new();
        if let Some(receiver) = &mut self.event_receiver {
            while let Ok(event) = receiver.try_recv() {
                pending_events.push(event);
            }
        }
        if !pending_events.is_empty() {
            redraw_update = true;
        }

        if tick_update {
            RUSTERIX.write().unwrap().client.inc_animation_frame();

            self.server_ctx.animation_counter = self.server_ctx.animation_counter.wrapping_add(1);
            // To update animated minimaps (only for docks that need it)
            if DOCKMANAGER
                .read()
                .unwrap()
                .current_dock_supports_minimap_animation()
            {
                ctx.ui.send(TheEvent::Custom(
                    TheId::named("Soft Update Minimap"),
                    TheValue::Empty,
                ));
            }

            if RUSTERIX.read().unwrap().server.state == rusterix::ServerState::Running {
                INFOVIEWER
                    .write()
                    .unwrap()
                    .update(&self.project, ui, ctx, &self.server_ctx);
            }
        }

        if redraw_update && !self.project.regions.is_empty() {
            SCENEMANAGER.write().unwrap().tick();

            self.build_values.set(
                "no_rect_geo",
                Value::Bool(self.server_ctx.no_rect_geo_on_map),
            );
            self.build_values
                .set("show_grid", Value::Bool(self.option_show_grid));
            self.build_values
                .set("show_gizmos", Value::Bool(self.option_show_gizmos));
            self.build_values
                .set("snap_to_grid", Value::Bool(self.option_snap_to_grid));

            extract_build_values_from_config(&mut self.build_values);

            let mut messages = Vec::new();
            let mut choices = Vec::new();

            // Update entities when the server is running
            {
                let rusterix = &mut RUSTERIX.write().unwrap();
                if rusterix.server.state == rusterix::ServerState::Running {
                    // Send a game tick to all servers
                    if tick_update {
                        rusterix.server.system_tick();
                    }

                    // Send a redraw tick to all servers
                    if redraw_update {
                        rusterix.server.redraw_tick();
                    }

                    if let Some(new_region_name) = rusterix.update_server() {
                        rusterix.client.current_map = new_region_name;
                    }
                    if rusterix.server.log_changed {
                        ui.set_widget_value(
                            "LogEdit",
                            ctx,
                            TheValue::Text(rusterix.server.get_log()),
                        );
                    }
                    for r in &mut self.project.regions {
                        rusterix.server.apply_entities_items(&mut r.map);

                        if r.id == self.server_ctx.curr_region {
                            if let Some(time) = rusterix.server.get_time(&r.map.id) {
                                rusterix.client.set_server_time(time);
                                if let Some(widget) = ui.get_widget("Server Time Slider") {
                                    widget.set_value(TheValue::Time(rusterix.client.server_time));
                                }
                            }

                            rusterix::tile_builder(&mut r.map, &mut rusterix.assets);
                            messages = rusterix.server.get_messages(&r.map.id);
                            choices = rusterix.server.get_choices(&r.map.id);

                            // Redraw the nodes
                            match &self.server_ctx.cc {
                                ContentContext::CharacterInstance(uuid) => {
                                    for entity in r.map.entities.iter() {
                                        if entity.creator_id == *uuid {
                                            CODEGRIDFX.write().unwrap().redraw_debug(
                                                ui,
                                                ctx,
                                                entity.id,
                                                &rusterix.server.debug,
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }

            if self.server_ctx.world_mode {
                // Draw World Editor
                WORLDEDITOR.write().unwrap().draw(
                    ui,
                    ctx,
                    &mut self.project,
                    &mut self.server_ctx,
                    &mut self.build_values,
                );
            } else {
                // Draw Map
                if let Some(render_view) = ui.get_render_view("PolyView") {
                    let dim = *render_view.dim();

                    let buffer = render_view.render_buffer_mut();
                    buffer.resize(dim.width, dim.height);

                    {
                        // If we are drawing billboard vertices in the geometry overlay, update them.
                        if !self.server_ctx.game_mode
                            && self.server_ctx.editor_view_mode != EditorViewMode::D2
                            && self.server_ctx.curr_map_tool_type == MapToolType::Vertex
                        {
                            TOOLLIST.write().unwrap().update_geometry_overlay_3d(
                                &mut self.project,
                                &mut self.server_ctx,
                            );
                        }

                        let rusterix = &mut RUSTERIX.write().unwrap();
                        let is_running = rusterix.server.state == rusterix::ServerState::Running;
                        let b = &mut rusterix.client.builder_d2;

                        if is_running && self.server_ctx.game_mode {
                            for r in &mut self.project.regions {
                                if r.map.name == rusterix.client.current_map {
                                    rusterix.draw_game(&r.map, messages, choices);
                                    break;
                                }
                            }

                            rusterix
                                .client
                                .insert_game_buffer(render_view.render_buffer_mut());
                        } else {
                            if self.server_ctx.editor_view_mode != EditorViewMode::D2
                                && self.server_ctx.get_map_context() == MapContext::Region
                            {
                                RENDEREDITOR.write().unwrap().draw(
                                    render_view,
                                    ctx,
                                    &mut self.project,
                                    &mut self.server_ctx,
                                    rusterix,
                                );
                            } else
                            // Draw the region map
                            if self.server_ctx.get_map_context() == MapContext::Region
                                && self.server_ctx.editing_surface.is_none()
                            {
                                if let Some(region) =
                                    self.project.get_region(&self.server_ctx.curr_region)
                                {
                                    b.set_clip_rect(None);
                                    b.set_map_tool_type(self.server_ctx.curr_map_tool_type);
                                    if let Some(hover_cursor) = self.server_ctx.hover_cursor {
                                        b.set_map_hover_info(
                                            self.server_ctx.hover,
                                            Some(vek::Vec2::new(hover_cursor.x, hover_cursor.y)),
                                        );
                                    } else {
                                        b.set_map_hover_info(self.server_ctx.hover, None);
                                    }

                                    if let Some(camera_pos) = region.map.camera_xz {
                                        b.set_camera_info(
                                            Some(Vec3::new(camera_pos.x, 0.0, camera_pos.y)),
                                            None,
                                        );
                                    }

                                    // let start_time = ctx.get_time();

                                    if let Some(clipboard) = &self.server_ctx.paste_clipboard {
                                        // During a paste operation we use a merged map

                                        let mut map = region.map.clone();
                                        if let Some(hover) = self.server_ctx.hover_cursor {
                                            map.paste_at_position(clipboard, hover);
                                        }

                                        rusterix.set_dirty();
                                        // rusterix.build_scene(
                                        //     Vec2::new(dim.width as f32, dim.height as f32),
                                        //     &map,
                                        //     &self.build_values,
                                        //     self.server_ctx.game_mode,
                                        // );
                                        rusterix.apply_entities_items(
                                            Vec2::new(dim.width as f32, dim.height as f32),
                                            &map,
                                            &self.server_ctx.editing_surface,
                                            false,
                                        );
                                    } else {
                                        // rusterix.build_scene(
                                        //     Vec2::new(dim.width as f32, dim.height as f32),
                                        //     &region.map,
                                        //     &self.build_values,
                                        //     self.server_ctx.game_mode,
                                        // );

                                        if let Some(map) = self.project.get_map(&self.server_ctx) {
                                            rusterix.apply_entities_items(
                                                Vec2::new(dim.width as f32, dim.height as f32),
                                                map,
                                                &self.server_ctx.editing_surface,
                                                false,
                                            );
                                        }
                                    }

                                    // Prepare the messages for the region for drawing
                                    rusterix.process_messages(&region.map, messages);

                                    // let stop_time = ctx.get_time();
                                    //println!("{} ms", stop_time - start_time);
                                }

                                if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
                                    rusterix.draw_scene(
                                        map,
                                        render_view.render_buffer_mut().pixels_mut(),
                                        dim.width as usize,
                                        dim.height as usize,
                                    );
                                }
                            } else if self.server_ctx.get_map_context() == MapContext::Region
                                && self.server_ctx.editing_surface.is_some()
                            {
                                b.set_map_tool_type(self.server_ctx.curr_map_tool_type);
                                if let Some(profile) = self.project.get_map_mut(&self.server_ctx) {
                                    if let Some(hover_cursor) = self.server_ctx.hover_cursor {
                                        b.set_map_hover_info(
                                            self.server_ctx.hover,
                                            Some(vek::Vec2::new(hover_cursor.x, hover_cursor.y)),
                                        );
                                    } else {
                                        b.set_map_hover_info(self.server_ctx.hover, None);
                                    }

                                    if let Some(clipboard) = &self.server_ctx.paste_clipboard {
                                        // During a paste operation we use a merged map
                                        let mut map = profile.clone();
                                        if let Some(hover) = self.server_ctx.hover_cursor {
                                            map.paste_at_position(clipboard, hover);
                                        }
                                        rusterix.set_dirty();
                                        rusterix.build_custom_scene_d2(
                                            Vec2::new(dim.width as f32, dim.height as f32),
                                            &map,
                                            &self.build_values,
                                            &self.server_ctx.editing_surface,
                                            true,
                                        );
                                        rusterix.draw_custom_d2(
                                            &map,
                                            render_view.render_buffer_mut().pixels_mut(),
                                            dim.width as usize,
                                            dim.height as usize,
                                        );
                                    } else {
                                        rusterix.build_custom_scene_d2(
                                            Vec2::new(dim.width as f32, dim.height as f32),
                                            profile,
                                            &self.build_values,
                                            &self.server_ctx.editing_surface,
                                            true,
                                        );
                                        rusterix.draw_custom_d2(
                                            profile,
                                            render_view.render_buffer_mut().pixels_mut(),
                                            dim.width as usize,
                                            dim.height as usize,
                                        );
                                    }
                                }
                            } else
                            // Draw the screen / character / item map
                            if self.server_ctx.get_map_context() == MapContext::Character
                                || self.server_ctx.get_map_context() == MapContext::Item
                                || self.server_ctx.get_map_context() == MapContext::Screen
                            {
                                b.set_map_tool_type(self.server_ctx.curr_map_tool_type);
                                if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
                                    if let Some(hover_cursor) = self.server_ctx.hover_cursor {
                                        b.set_map_hover_info(
                                            self.server_ctx.hover,
                                            Some(vek::Vec2::new(hover_cursor.x, hover_cursor.y)),
                                        );
                                    } else {
                                        b.set_map_hover_info(self.server_ctx.hover, None);
                                    }

                                    if self.server_ctx.get_map_context() != MapContext::Screen {
                                        b.set_clip_rect(Some(rusterix::Rect {
                                            x: -5.0,
                                            y: -5.0,
                                            width: 10.0,
                                            height: 10.0,
                                        }));
                                    } else {
                                        let viewport = CONFIGEDITOR.read().unwrap().viewport;
                                        let grid_size =
                                            CONFIGEDITOR.read().unwrap().grid_size as f32;
                                        let w = viewport.x as f32 / grid_size;
                                        let h = viewport.y as f32 / grid_size;
                                        b.set_clip_rect(Some(rusterix::Rect {
                                            x: -w / 2.0,
                                            y: -h / 2.0,
                                            width: w,
                                            height: h,
                                        }));
                                    }

                                    if let Some(clipboard) = &self.server_ctx.paste_clipboard {
                                        // During a paste operation we use a merged map
                                        let mut map = map.clone();
                                        if let Some(hover) = self.server_ctx.hover_cursor {
                                            map.paste_at_position(clipboard, hover);
                                        }
                                        rusterix.set_dirty();
                                        rusterix.build_custom_scene_d2(
                                            Vec2::new(dim.width as f32, dim.height as f32),
                                            &map,
                                            &self.build_values,
                                            &self.server_ctx.editing_surface,
                                            true,
                                        );
                                        rusterix.draw_custom_d2(
                                            &map,
                                            render_view.render_buffer_mut().pixels_mut(),
                                            dim.width as usize,
                                            dim.height as usize,
                                        );
                                    } else {
                                        rusterix.build_custom_scene_d2(
                                            Vec2::new(dim.width as f32, dim.height as f32),
                                            map,
                                            &self.build_values,
                                            &None,
                                            true,
                                        );
                                        rusterix.draw_custom_d2(
                                            map,
                                            render_view.render_buffer_mut().pixels_mut(),
                                            dim.width as usize,
                                            dim.height as usize,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    if !self.server_ctx.game_mode {
                        if let Some(map) = self.project.get_map_mut(&self.server_ctx) {
                            TOOLLIST.write().unwrap().draw_hud(
                                render_view.render_buffer_mut(),
                                map,
                                ctx,
                                &mut self.server_ctx,
                                &RUSTERIX.read().unwrap().assets,
                            );
                        }
                    }
                }
            }

            // Draw the 3D Preview if active.
            // if !self.server_ctx.game_mode
            //     && self.server_ctx.curr_map_tool_helper == MapToolHelper::Preview
            // {
            //     if let Some(region) = self.project.get_region_ctx(&self.server_ctx) {
            //         PREVIEWVIEW
            //             .write()
            //             .unwrap()
            //             .draw(region, ui, ctx, &mut self.server_ctx);
            //     }
            // }

            redraw = true;
        }

        for event in pending_events {
            if self.server_ctx.game_input_mode {
                // In game input mode send events to the game tool
                if let Some(game_tool) =
                    TOOLLIST.write().unwrap().get_game_tool_of_name("Game Tool")
                {
                    redraw = game_tool.handle_event(
                        &event,
                        ui,
                        ctx,
                        &mut self.project,
                        &mut self.server_ctx,
                    );
                }
            }
            if self
                .sidebar
                .handle_event(&event, ui, ctx, &mut self.project, &mut self.server_ctx)
            {
                redraw = true;
            }
            if TOOLLIST.write().unwrap().handle_event(
                &event,
                ui,
                ctx,
                &mut self.project,
                &mut self.server_ctx,
            ) {
                redraw = true;
            }
            if DOCKMANAGER.write().unwrap().handle_event(
                &event,
                ui,
                ctx,
                &mut self.project,
                &mut self.server_ctx,
            ) {
                redraw = true;
            }
            if self
                .mapeditor
                .handle_event(&event, ui, ctx, &mut self.project, &mut self.server_ctx)
            {
                redraw = true;
            }
            if TILEMAPEDITOR.write().unwrap().handle_event(
                &event,
                ui,
                ctx,
                &mut self.project,
                &mut self.server_ctx,
            ) {
                redraw = true;
            }
            match event {
                TheEvent::ContextMenuSelected(_widget_id, item_id) => {
                    if item_id.name.starts_with("MenuTool::") {
                        if let Some(tool_name) = item_id.name.strip_prefix("MenuTool::") {
                            ctx.ui.send(TheEvent::Custom(
                                TheId::named("Set Tool"),
                                TheValue::Text(tool_name.to_string()),
                            ));
                            redraw = true;
                        }
                    } else if item_id.name == "MenuFile::New" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("New"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuFile::Open" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Open"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuFile::Save" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Save"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuFile::SaveAs" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Save As"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuEdit::Undo" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Undo"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuEdit::Redo" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Redo"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuBuild::Play" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Play"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuBuild::Pause" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Pause"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuBuild::Stop" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Stop"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuTools::GenerateTown" {
                        self.generate_town_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuTools::GenerateFantasyWorld" {
                        self.generate_fantasy_world_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuTownPreset::SmallTown" {
                        self.towngen_preset = "Small Town".to_string();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Town preset set: Small Town".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTownPreset::LargeTown" {
                        self.towngen_preset = "Large Town".to_string();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Town preset set: Large Town".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTownPreset::SmallCity" {
                        self.towngen_preset = "Small City".to_string();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Town preset set: Small City".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTownPreset::LargeCity" {
                        self.towngen_preset = "Large City".to_string();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Town preset set: Large City".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTown::ReseedGenerate" {
                        let seed = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(1);
                        self.generate_town_system_data_with_seed(ui, ctx, seed);
                        redraw = true;
                    } else if item_id.name == "MenuTown::Regenerate" {
                        let seed = if self.towngen_last_seed == 0 {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(1)
                        } else {
                            self.towngen_last_seed
                        };
                        self.generate_town_system_data_with_seed(ui, ctx, seed);
                        redraw = true;
                    } else if item_id.name == "MenuTown::ToggleRiver" {
                        self.towngen_has_river = !self.towngen_has_river;
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            format!("Town river option: {}", self.towngen_has_river),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTown::ToggleWalls" {
                        self.towngen_has_walls = !self.towngen_has_walls;
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            format!("Town walls option: {}", self.towngen_has_walls),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTown::BakeMap" {
                        self.bake_generated_town_to_current_map(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuTown::ExportJson" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("ExportTownJson", Uuid::new_v4()),
                            "Export Town Data".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuTown::ImportJson" {
                        ctx.ui.open_file_requester(
                            TheId::named_with_id("ImportTownJson", Uuid::new_v4()),
                            "Import Town Data".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuTown::AutoQuest" {
                        let n = self.generate_quests_from_last_town();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            format!("Generated {} MMO quests from town districts.", n),
                        ));
                    } else if item_id.name == "MenuTown::SpawnPoi" {
                        let n = self.spawn_pois_from_last_town();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            format!("Spawned {} town POI NPCs/entities.", n),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuTools::GenerateRpgMmorpg" {
                        self.generate_rpg_mmorpg_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuMmorpgBuilder::Open" {
                        self.show_rpg_mmorpg_builder_dialog(ui, ctx);
                    } else if item_id.name == "MenuMmorpgBuilder::Generate" {
                        self.generate_rpg_mmorpg_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuMmoSim::Tick" {
                        let msg = self.run_mmorpg_sim_tick();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuMmoSim::Combat" {
                        let msg = self.run_mmorpg_sim_combat();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuMmoSim::Loot" {
                        let msg = self.run_mmorpg_sim_loot();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuMapForge::OpenEditor" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("World Tool".to_string()),
                        ));
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Map Forge editor opened (World Tool active).".to_string(),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ToolSelect" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Select Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ToolTerrain" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("World Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ToolRoad" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Linedef Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ToolDistrict" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Sector Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ToolLandmark" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Entity Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::LayerDistricts" {
                        self.overlay_show_town_districts = !self.overlay_show_town_districts;
                        self.apply_town_overlay_visibility();
                        ui.set_widget_value(
                            "OverlayTownDistrictsCB",
                            ctx,
                            TheValue::Bool(self.overlay_show_town_districts),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuMapForge::LayerRoads" {
                        self.overlay_show_town_roads = !self.overlay_show_town_roads;
                        self.apply_town_overlay_visibility();
                        ui.set_widget_value(
                            "OverlayTownRoadsCB",
                            ctx,
                            TheValue::Bool(self.overlay_show_town_roads),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuMapForge::LayerLandmarks" {
                        self.overlay_show_town_landmarks = !self.overlay_show_town_landmarks;
                        self.apply_town_overlay_visibility();
                        ui.set_widget_value(
                            "OverlayTownLandmarksCB",
                            ctx,
                            TheValue::Bool(self.overlay_show_town_landmarks),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuMapForge::GenerateTown" {
                        self.generate_town_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::GenerateFantasyWorld" {
                        self.generate_fantasy_world_system_data(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ReseedTown" {
                        let seed = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .map(|d| d.as_secs())
                            .unwrap_or(1);
                        self.generate_town_system_data_with_seed(ui, ctx, seed);
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::RegenerateTown" {
                        let seed = if self.towngen_last_seed == 0 {
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .map(|d| d.as_secs())
                                .unwrap_or(1)
                        } else {
                            self.towngen_last_seed
                        };
                        self.generate_town_system_data_with_seed(ui, ctx, seed);
                        redraw = true;
                    } else if item_id.name == "MenuMapForge::ExportJson" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("ExportTownJson", Uuid::new_v4()),
                            "Export Town Data".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuMapForge::ImportJson" {
                        ctx.ui.open_file_requester(
                            TheId::named_with_id("ImportTownJson", Uuid::new_v4()),
                            "Import Town Data".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuMapForge::BakeMap" {
                        self.bake_generated_town_to_current_map(ui, ctx);
                        redraw = true;
                    } else if item_id.name == "MenuSelect::All" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Select All requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuSelect::None" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Selection cleared.".to_string(),
                        ));
                    } else if item_id.name == "MenuSelect::Invert" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Invert selection requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuSelect::FilterGeometry" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Selection filter: geometry.".to_string(),
                        ));
                    } else if item_id.name == "MenuSelect::FilterEntity" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Selection filter: entities.".to_string(),
                        ));
                    } else if item_id.name == "MenuSelect::FilterLandmark" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Selection filter: landmarks.".to_string(),
                        ));
                    } else if item_id.name == "MenuActor::PlaceEntity" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Entity Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuActor::PlaceLandmark" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Entity Tool".to_string()),
                        ));
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Landmark placement mode (Entity Tool) active.".to_string(),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuActor::PlaceTerrain" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("World Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuActor::AlignToGrid" {
                        self.option_snap_to_grid = true;
                        self.server_ctx.snap_to_grid = true;
                        ui.set_widget_value("OptionSnapCB", ctx, TheValue::Bool(true));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuActor::SnapToGrid" {
                        self.option_snap_to_grid = !self.option_snap_to_grid;
                        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
                        ui.set_widget_value(
                            "OptionSnapCB",
                            ctx,
                            TheValue::Bool(self.option_snap_to_grid),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuActor::FocusSelection" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Focus selection requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuBlueprint::OpenPanel" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::Blueprint,
                        );
                        let launch = crate::features::launch_blueprint_editor();
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            launch.status_line,
                        ));
                    } else if item_id.name == "MenuBlueprint::CreateCharacter" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("CreateCharacterBlueprint", Uuid::new_v4()),
                            "Create Character Blueprint".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuBlueprint::CreateItem" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("CreateItemBlueprint", Uuid::new_v4()),
                            "Create Item Blueprint".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuBlueprint::CreateQuest" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("CreateQuestBlueprint", Uuid::new_v4()),
                            "Create Quest Blueprint".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuBlueprint::CompileAll" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Blueprint compile-all requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuBlueprint::Validate" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Blueprint validation requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuCinematics::CreateSequence" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("CreateLevelSequence", Uuid::new_v4()),
                            "Create Level Sequence".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuCinematics::AddCameraTrack" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Camera track add requested.".to_string(),
                        ));
                    } else if item_id.name == "MenuCinematics::PlaySequence" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Play"),
                            TheWidgetState::Clicked,
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::Select" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Select Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::Landscape" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("World Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::Modeling" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Rect Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::Foliage" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Entity Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::BrushEditing" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Sector Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::Animation" {
                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Set Tool"),
                            TheValue::Text("Code Tool".to_string()),
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuMode::GameView" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("GameInput"),
                            TheWidgetState::Clicked,
                        ));
                        redraw = true;
                    } else if item_id.name == "MenuPlatforms::PackageWindows" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("PackageWindows", Uuid::new_v4()),
                            "Package for Windows".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuPlatforms::PackageLinux" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("PackageLinux", Uuid::new_v4()),
                            "Package for Linux".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuPlatforms::PackageMac" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("PackageMac", Uuid::new_v4()),
                            "Package for macOS".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuPlatforms::PackageWeb" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("PackageWeb", Uuid::new_v4()),
                            "Package for Web".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuPlatforms::ProjectSettings" {
                        self.show_ide_feature_matrix_dialog(ui, ctx);
                    } else if item_id.name == "MenuPlatforms::DeviceManager" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Device manager opened (stub).".to_string(),
                        ));
                    } else if item_id.name == "MenuPlatforms::SdkManager" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "SDK manager opened (stub).".to_string(),
                        ));
                    } else if item_id.name == "MenuLayouts::Unreal" {
                        self.apply_ide_layout_unreal(ui, ctx);
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuLayouts::Minimal" {
                        self.apply_ide_layout_minimal(ui, ctx);
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuLayouts::FeatureMatrix" {
                        self.show_ide_feature_matrix_dialog(ui, ctx);
                    } else if item_id.name == "MenuDebug::ToggleRuntime" {
                        if self.is_realtime_mode() {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Stop"),
                                TheWidgetState::Clicked,
                            ));
                        } else {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Play"),
                                TheWidgetState::Clicked,
                            ));
                        }
                        redraw = true;
                    } else if item_id.name == "MenuDebug::SimTick" {
                        let msg = self.run_mmorpg_sim_tick();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuDebug::SimCombat" {
                        let msg = self.run_mmorpg_sim_combat();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuDebug::SimLoot" {
                        let msg = self.run_mmorpg_sim_loot();
                        ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                    } else if item_id.name == "MenuDebug::FeatureMatrix" {
                        self.show_ide_feature_matrix_dialog(ui, ctx);
                    } else if item_id.name == "MenuSource::Connect" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Source control connect requested (stub).".to_string(),
                        ));
                    } else if item_id.name == "MenuSource::SubmitContent" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("SourceSubmitContent", Uuid::new_v4()),
                            "Source Control Submit".into(),
                            TheFileExtension::new("TXT".into(), vec!["txt".to_string()]),
                        );
                    } else if item_id.name == "MenuSource::Sync" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Source control sync requested (stub).".to_string(),
                        ));
                    } else if item_id.name == "MenuSource::RevertUnchanged" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Source control revert unchanged requested (stub).".to_string(),
                        ));
                    } else if item_id.name == "MenuView::ToggleLeft" {
                        self.show_left_toolbar = !self.show_left_toolbar;
                        self.apply_toolbar_visibility(ui, ctx);
                        ui.set_widget_value(
                            "OptionLeftToolbarCB",
                            ctx,
                            TheValue::Bool(self.show_left_toolbar),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuView::ToggleRight" {
                        self.show_right_toolbar = !self.show_right_toolbar;
                        self.apply_toolbar_visibility(ui, ctx);
                        ui.set_widget_value(
                            "OptionRightToolbarCB",
                            ctx,
                            TheValue::Bool(self.show_right_toolbar),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTheme::Dark" {
                        self.theme_preset = "Dark".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(0));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTheme::Light" {
                        self.theme_preset = "Light".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(1));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuTheme::Slate" {
                        self.theme_preset = "Slate".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(2));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuOption::Snap" {
                        self.option_snap_to_grid = !self.option_snap_to_grid;
                        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
                        ui.set_widget_value(
                            "OptionSnapCB",
                            ctx,
                            TheValue::Bool(self.option_snap_to_grid),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuOption::Grid" {
                        self.option_show_grid = !self.option_show_grid;
                        ui.set_widget_value(
                            "OptionGridCB",
                            ctx,
                            TheValue::Bool(self.option_show_grid),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuOption::Gizmos" {
                        self.option_show_gizmos = !self.option_show_gizmos;
                        self.server_ctx.show_editing_geometry = self.option_show_gizmos;
                        ui.set_widget_value(
                            "OptionGizmoCB",
                            ctx,
                            TheValue::Bool(self.option_show_gizmos),
                        );
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuIde::LayoutUnreal" {
                        self.apply_ide_layout_unreal(ui, ctx);
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Applied Unreal-like IDE layout preset.".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuIde::LayoutMinimal" {
                        self.apply_ide_layout_minimal(ui, ctx);
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Applied minimal IDE layout preset.".to_string(),
                        ));
                        workspace_prefs_dirty = true;
                    } else if item_id.name == "MenuIde::FeatureMatrix" {
                        self.show_ide_feature_matrix_dialog(ui, ctx);
                    } else if item_id.name == "MenuWindow::ContentBrowser" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::ContentBrowser,
                        );
                    } else if item_id.name == "MenuWindow::WorldOutliner" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::WorldOutliner,
                        );
                    } else if item_id.name == "MenuWindow::Details" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::Details,
                        );
                    } else if item_id.name == "MenuWindow::OutputLog" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::OutputLog,
                        );
                    } else if item_id.name == "MenuWindow::Blueprint" {
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::Blueprint,
                        );
                    } else if item_id.name == "MenuHelp::Docs" {
                        self.show_help_dialog(ui, ctx);
                    } else if item_id.name == "MenuHelp::Examples" {
                        _ = open::that("https://www.eldiron.com/docs/games");
                    } else if item_id.name == "MenuHelp::About" {
                        self.show_about_dialog(ui, ctx);
                    } else if item_id.name == "MenuBuild::Export2D" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("Export2DPackage", Uuid::new_v4()),
                            "Export 2D Package".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuBuild::Export3D" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id("Export3DPackage", Uuid::new_v4()),
                            "Export 3D Package".into(),
                            TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                        );
                    } else if item_id.name == "MenuEdit::Copy" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Copy"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuEdit::Paste" {
                        ctx.ui.send(TheEvent::StateChanged(
                            TheId::named("Paste"),
                            TheWidgetState::Clicked,
                        ));
                    } else if item_id.name == "MenuFile::Close" {
                        self.project_path = None;
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Project closed (path cleared).".to_string(),
                        ));
                    }
                }
                TheEvent::CustomUndo(id, p, n) => {
                    if id.name == "ModuleUndo" {
                        if CODEEDITOR.read().unwrap().active_panel == VisibleCodePanel::Shade {
                            let prev = Module::from_json(&p);
                            let next = Module::from_json(&n);

                            let atom = MaterialUndoAtom::ShaderEdit(prev, next);
                            UNDOMANAGER.write().unwrap().add_material_undo(atom, ctx);
                        } else if CODEEDITOR.read().unwrap().active_panel == VisibleCodePanel::Code
                        {
                            let prev = Module::from_json(&p);
                            let next = Module::from_json(&n);
                            match CODEEDITOR.read().unwrap().code_content {
                                ContentContext::CharacterTemplate(id) => {
                                    let atom =
                                        CharacterUndoAtom::TemplateModuleEdit(id, prev, next);
                                    UNDOMANAGER.write().unwrap().add_character_undo(atom, ctx);
                                }
                                ContentContext::CharacterInstance(id) => {
                                    let atom = CharacterUndoAtom::InstanceModuleEdit(
                                        self.server_ctx.curr_region,
                                        id,
                                        prev,
                                        next,
                                    );
                                    UNDOMANAGER.write().unwrap().add_character_undo(atom, ctx);
                                }
                                ContentContext::ItemTemplate(id) => {
                                    let atom: ItemUndoAtom =
                                        ItemUndoAtom::TemplateModuleEdit(id, prev, next);
                                    UNDOMANAGER.write().unwrap().add_item_undo(atom, ctx);
                                }
                                ContentContext::ItemInstance(id) => {
                                    let atom = ItemUndoAtom::InstanceModuleEdit(
                                        self.server_ctx.curr_region,
                                        id,
                                        prev,
                                        next,
                                    );
                                    UNDOMANAGER.write().unwrap().add_item_undo(atom, ctx);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                TheEvent::Custom(id, value) => {
                    if id.name == "Show Help" {
                        if let TheValue::Text(ref url) = value {
                            _ = open::that(format!("https://www.eldiron.com/{}", url));
                            ctx.ui
                                .set_widget_state("Help".to_string(), TheWidgetState::None);
                            ctx.ui.clear_hover();
                            self.server_ctx.help_mode = false;
                            redraw = true;
                        }
                    }
                    if id.name == "Set Project Undo State" {
                        UNDOMANAGER.read().unwrap().set_undo_state_to_ui(ctx);
                    } else if id.name == "Rebuild Left Toolbar" {
                        self.rebuild_left_tool_name_layout(ui, ctx);
                        self.persist_workspace_settings_to_project_config();
                        redraw = true;
                    } else if id.name == "Set Tool" {
                        if let TheValue::Text(ref tool_name) = value {
                            self.set_left_toolbar_active_tool(ctx, tool_name);
                        }
                    } else if id.name == "Render SceneManager Map" {
                        if self.server_ctx.pc.is_region() {
                            if self.server_ctx.editor_view_mode == EditorViewMode::D2
                                && self.server_ctx.profile_view.is_some()
                            {
                            } else {
                                crate::utils::scenemanager_render_map(
                                    &self.project,
                                    &self.server_ctx,
                                );
                                if self.server_ctx.editor_view_mode != EditorViewMode::D2 {
                                    TOOLLIST.write().unwrap().update_geometry_overlay_3d(
                                        &mut self.project,
                                        &mut self.server_ctx,
                                    );
                                }
                            }
                        }
                    } else if id.name == "Tool Changed" {
                        TOOLLIST
                            .write()
                            .unwrap()
                            .update_geometry_overlay_3d(&mut self.project, &mut self.server_ctx);
                    } else if id.name == "Update Client Properties" {
                        let mut rusterix = RUSTERIX.write().unwrap();
                        self.build_values.set(
                            "no_rect_geo",
                            rusterix::Value::Bool(self.server_ctx.no_rect_geo_on_map),
                        );
                        self.build_values.set(
                            "editing_slice",
                            rusterix::Value::Float(self.server_ctx.editing_slice),
                        );
                        rusterix
                            .client
                            .builder_d2
                            .set_properties(&self.build_values);
                        rusterix.set_dirty();
                    }
                }

                TheEvent::DialogValueOnClose(role, name, uuid, _value) => {
                    if name == "Delete Character Instance ?" {
                        if role == TheDialogButtonRole::Delete {
                            if let Some(region) =
                                self.project.get_region_mut(&self.server_ctx.curr_region)
                            {
                                let character_id = uuid;
                                if region.characters.shift_remove(&character_id).is_some() {
                                    self.server_ctx.curr_region_content = ContentContext::Unknown;
                                    region.map.selected_entity_item = None;
                                    redraw = true;

                                    // Remove from the content list
                                    if let Some(list) = ui.get_list_layout("Region Content List") {
                                        list.remove(TheId::named_with_id(
                                            "Region Content List Item",
                                            character_id,
                                        ));
                                        ui.select_first_list_item("Region Content List", ctx);
                                        ctx.ui.relayout = true;
                                    }
                                    insert_content_into_maps(&mut self.project);
                                    RUSTERIX.write().unwrap().set_dirty();
                                }
                            }
                        }
                    } else if name == "Delete Item Instance ?" {
                        if role == TheDialogButtonRole::Delete {
                            if let Some(region) =
                                self.project.get_region_mut(&self.server_ctx.curr_region)
                            {
                                let item_id = uuid;
                                if region.items.shift_remove(&item_id).is_some() {
                                    self.server_ctx.curr_region_content = ContentContext::Unknown;
                                    redraw = true;

                                    // Remove from the content list
                                    if let Some(list) = ui.get_list_layout("Region Content List") {
                                        list.remove(TheId::named_with_id(
                                            "Region Content List Item",
                                            item_id,
                                        ));
                                        ui.select_first_list_item("Region Content List", ctx);
                                        ctx.ui.relayout = true;
                                    }
                                    insert_content_into_maps(&mut self.project);
                                    RUSTERIX.write().unwrap().set_dirty();
                                }
                            }
                        }
                    } else if name == "Update Eldiron" && role == TheDialogButtonRole::Accept {
                        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
                        {
                            let updater = self.self_updater.lock().unwrap();

                            if updater.has_newer_release() {
                                let release = updater.latest_release().cloned().unwrap();

                                let updater = Arc::clone(&self.self_updater);
                                let tx = self.self_update_tx.clone();

                                self.self_update_tx
                                    .send(SelfUpdateEvent::UpdateStart(release.clone()))
                                    .unwrap();

                                thread::spawn(move || {
                                    match updater.lock().unwrap().update_latest() {
                                        Ok(status) => match status {
                                            self_update::Status::UpToDate(_) => {
                                                tx.send(SelfUpdateEvent::AlreadyUpToDate).unwrap();
                                            }
                                            self_update::Status::Updated(_) => {
                                                tx.send(SelfUpdateEvent::UpdateCompleted(release))
                                                    .unwrap();
                                            }
                                        },
                                        Err(err) => {
                                            tx.send(SelfUpdateEvent::UpdateError(err.to_string()))
                                                .unwrap();
                                        }
                                    }
                                });
                            } else {
                                self.self_update_tx
                                    .send(SelfUpdateEvent::AlreadyUpToDate)
                                    .unwrap();
                            }
                        }
                    }
                }
                TheEvent::RenderViewDrop(_id, location, drop) => {
                    if drop.id.name.starts_with("Shader") {
                        if self.server_ctx.curr_map_tool_helper == MapToolHelper::ShaderEditor
                            && CODEEDITOR.read().unwrap().active_panel == VisibleCodePanel::Shade
                        {
                            if matches!(
                                CODEEDITOR.read().unwrap().shader_content,
                                ContentContext::Sector(_)
                            ) {
                                if let Some(shader) = self.project.shaders.get(&drop.id.uuid) {
                                    let prev = SHADEGRIDFX.read().unwrap().clone();
                                    if SHADEGRIDFX.write().unwrap().insert_module(shader, location)
                                    {
                                        ctx.ui.send(TheEvent::Custom(
                                            TheId::named("ModuleChanged"),
                                            TheValue::Empty,
                                        ));
                                        ctx.ui.send(TheEvent::CustomUndo(
                                            TheId::named("ModuleUndo"),
                                            prev.to_json(),
                                            SHADEGRIDFX.read().unwrap().to_json(),
                                        ));
                                    }
                                }
                            }
                        }

                        return true;
                    }

                    let mut grid_pos = Vec2::zero();

                    if let Some(map) = self.project.get_map(&self.server_ctx) {
                        if let Some(render_view) = ui.get_render_view("PolyView") {
                            let dim = *render_view.dim();
                            grid_pos = self.server_ctx.local_to_map_cell(
                                Vec2::new(dim.width as f32, dim.height as f32),
                                Vec2::new(location.x as f32, location.y as f32),
                                map,
                                map.subdivisions,
                            );
                            grid_pos += 0.5;
                        }
                    }

                    if drop.id.name.starts_with("Character") {
                        let mut instance = Character {
                            character_id: drop.id.references,
                            position: Vec3::new(grid_pos.x, 1.5, grid_pos.y),
                            ..Default::default()
                        };

                        if let Some(bytes) = crate::Embedded::get("python/instcharacter.py") {
                            if let Ok(source) = std::str::from_utf8(bytes.data.as_ref()) {
                                instance.source = source.to_string();
                            }
                        }

                        let mut name = "Character".to_string();
                        if let Some(character) = self.project.characters.get(&drop.id.references) {
                            name.clone_from(&character.name);
                        }
                        instance.name = name.clone();

                        let atom = ProjectUndoAtom::AddRegionCharacterInstance(
                            self.server_ctx.curr_region,
                            instance,
                        );
                        atom.redo(&mut self.project, ui, ctx, &mut self.server_ctx);
                        UNDOMANAGER.write().unwrap().add_undo(atom, ctx);
                    } else if drop.id.name.starts_with("Item") {
                        let mut instance = Item {
                            item_id: drop.id.references,
                            position: Vec3::new(grid_pos.x, 1.5, grid_pos.y),
                            ..Default::default()
                        };

                        if let Some(bytes) = crate::Embedded::get("python/institem.py") {
                            if let Ok(source) = std::str::from_utf8(bytes.data.as_ref()) {
                                instance.source = source.to_string();
                            }
                        }

                        let mut name = "Item".to_string();
                        if let Some(item) = self.project.items.get(&drop.id.references) {
                            name.clone_from(&item.name);
                        }
                        instance.name = name;

                        let atom = ProjectUndoAtom::AddRegionItemInstance(
                            self.server_ctx.curr_region,
                            instance,
                        );
                        atom.redo(&mut self.project, ui, ctx, &mut self.server_ctx);
                        UNDOMANAGER.write().unwrap().add_undo(atom, ctx);
                    }
                }
                /*
                TheEvent::TileEditorDrop(_id, location, drop) => {
                    if drop.id.name.starts_with("Character") {
                        let mut instance = TheCodeBundle::new();

                        let mut init = TheCodeGrid {
                            name: "init".into(),
                            ..Default::default()
                        };
                        init.insert_atom(
                            (0, 0),
                            TheCodeAtom::Set(
                                "@self.position".to_string(),
                                TheValueAssignment::Assign,
                            ),
                        );
                        init.insert_atom(
                            (1, 0),
                            TheCodeAtom::Assignment(TheValueAssignment::Assign),
                        );
                        init.insert_atom(
                            (2, 0),
                            TheCodeAtom::Value(TheValue::Position(Vec3::new(
                                location.x as f32,
                                0.0,
                                location.y as f32,
                            ))),
                        );
                        instance.insert_grid(init);

                        // Set the character instance bundle, disabled for now

                        // self.sidebar.code_editor.set_bundle(
                        //     instance.clone(),
                        //     ctx,
                        //     self.sidebar.width,
                        // );

                        let character = Character {
                            id: instance.id,
                            character_id: drop.id.uuid,
                            instance,
                        };

                        // Add the character instance to the region content list

                        let mut name = "Character".to_string();
                        if let Some(character) = self.project.characters.get(&drop.id.uuid) {
                            name.clone_from(&character.name);
                        }

                        if let Some(list) = ui.get_list_layout("Region Content List") {
                            let mut item = TheListItem::new(TheId::named_with_id(
                                "Region Content List Item",
                                character.id,
                            ));
                            item.set_text(name);
                            item.set_state(TheWidgetState::Selected);
                            item.add_value_column(100, TheValue::Text("Character".to_string()));

                            list.deselect_all();
                            item.set_context_menu(Some(TheContextMenu {
                                items: vec![TheContextMenuItem::new(
                                    "Delete Character...".to_string(),
                                    TheId::named("Sidebar Delete Character Instance"),
                                )],
                                ..Default::default()
                            }));
                            list.add_item(item, ctx);
                            list.select_item(character.id, ctx, true);
                        }

                        // Add the character instance to the project

                        if let Some(region) =
                            self.project.get_region_mut(&self.server_ctx.curr_region)
                        {
                            region.characters.insert(character.id, character.clone());
                        }

                        // Add the character instance to the server

                        self.server_ctx.curr_character = Some(character.character_id);
                        self.server_ctx.curr_character_instance = Some(character.id);
                        self.server_ctx.curr_area = None;
                        //self.sidebar.deselect_all("Character List", ui);

                        self.server_ctx.curr_grid_id =
                            self.server.add_character_instance_to_region(
                                self.server_ctx.curr_region,
                                character,
                                None,
                            );

                        // Set the character instance debug info, disabled for now

                        // if let Some(curr_grid_id) = self.server_ctx.curr_grid_id {
                        //     let debug_module = self.server.get_region_debug_module(
                        //         self.server_ctx.curr_region,
                        //         curr_grid_id,
                        //     );

                        //     self.sidebar.code_editor.set_debug_module(debug_module, ui);
                        // }
                    } else if drop.id.name.starts_with("Item") {
                        let mut instance = TheCodeBundle::new();

                        let mut init = TheCodeGrid {
                            name: "init".into(),
                            ..Default::default()
                        };
                        init.insert_atom(
                            (0, 0),
                            TheCodeAtom::Set(
                                "@self.position".to_string(),
                                TheValueAssignment::Assign,
                            ),
                        );
                        init.insert_atom(
                            (1, 0),
                            TheCodeAtom::Assignment(TheValueAssignment::Assign),
                        );
                        init.insert_atom(
                            (2, 0),
                            TheCodeAtom::Value(TheValue::Position(Vec3::new(
                                location.x as f32,
                                0.0,
                                location.y as f32,
                            ))),
                        );
                        instance.insert_grid(init);

                        // Set the character instance bundle, disabled for now

                        // self.sidebar.code_editor.set_bundle(
                        //     instance.clone(),
                        //     ctx,
                        //     self.sidebar.width,
                        // );

                        let item = Item {
                            id: instance.id,
                            item_id: drop.id.uuid,
                            instance,
                        };

                        // Add the item instance to the region content list

                        let mut name = "Item".to_string();
                        if let Some(item) = self.project.items.get(&drop.id.uuid) {
                            name.clone_from(&item.name);
                        }

                        if let Some(list) = ui.get_list_layout("Region Content List") {
                            let mut list_item = TheListItem::new(TheId::named_with_id(
                                "Region Content List Item",
                                item.id,
                            ));
                            list_item.set_text(name);
                            list_item.set_state(TheWidgetState::Selected);
                            list_item.add_value_column(100, TheValue::Text("Item".to_string()));

                            list.deselect_all();
                            list.add_item(list_item, ctx);
                            list.select_item(item.id, ctx, true);
                        }

                        // Add the item instance to the project

                        if let Some(region) =
                            self.project.get_region_mut(&self.server_ctx.curr_region)
                        {
                            region.items.insert(item.id, item.clone());
                        }

                        // Add the character instance to the server

                        self.server_ctx.curr_character = None;
                        self.server_ctx.curr_character_instance = None;
                        self.server_ctx.curr_item = Some(item.item_id);
                        self.server_ctx.curr_item_instance = Some(item.id);
                        self.server_ctx.curr_area = None;

                        self.server_ctx.curr_grid_id = self
                            .server
                            .add_item_instance_to_region(self.server_ctx.curr_region, item);

                        // Set the character instance debug info, disabled for now

                        // if let Some(curr_grid_id) = self.server_ctx.curr_grid_id {
                        //     let debug_module = self.server.get_region_debug_module(
                        //         self.server_ctx.curr_region,
                        //         curr_grid_id,
                        //     );

                        //     self.sidebar.code_editor.set_debug_module(debug_module, ui);
                        // }
                    }
                }*/
                TheEvent::FileRequesterResult(id, paths) => {
                    // Load a palette from a file
                    if id.name == "Palette Import" {
                        for p in paths {
                            let contents = std::fs::read_to_string(p).unwrap_or("".to_string());
                            let prev = self.project.palette.clone();
                            self.project.palette.load_from_txt(contents);
                            *PALETTE.write().unwrap() = self.project.palette.clone();

                            if let Some(palette_picker) = ui.get_palette_picker("Palette Picker") {
                                let index = palette_picker.index();

                                palette_picker.set_palette(self.project.palette.clone());
                                if let Some(widget) = ui.get_widget("Palette Color Picker") {
                                    if let Some(color) = &self.project.palette[index] {
                                        widget.set_value(TheValue::ColorObject(color.clone()));
                                    }
                                }
                                if let Some(widget) = ui.get_widget("Palette Hex Edit") {
                                    if let Some(color) = &self.project.palette[index] {
                                        widget.set_value(TheValue::Text(color.to_hex()));
                                    }
                                }
                            }
                            redraw = true;

                            let undo = PaletteUndoAtom::Edit(prev, self.project.palette.clone());
                            UNDOMANAGER.write().unwrap().add_palette_undo(undo, ctx);
                        }
                    } else if id.name == "ExportTownJson" {
                        if let Some(generated) = &self.last_generated_town {
                            let payload = serde_json::to_string_pretty(generated)
                                .unwrap_or_else(|_| "{}".to_string());
                            for p in paths {
                                let _ = std::fs::write(&p, &payload);
                            }
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "Town data exported.".to_string(),
                            ));
                        } else {
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "No generated town data to export.".to_string(),
                            ));
                        }
                    } else if id.name == "ImportTownJson" {
                        for p in paths {
                            if let Ok(text) = std::fs::read_to_string(&p)
                                && let Ok(data) =
                                    serde_json::from_str::<crate::game_logic::TownMapData>(&text)
                            {
                                self.last_generated_town = Some(data);
                                ctx.ui.send(TheEvent::SetStatusText(
                                    TheId::empty(),
                                    "Town data imported.".to_string(),
                                ));
                            }
                        }
                    } else if id.name == "Export2DPackage" || id.name == "Export3DPackage" {
                        let export_kind = if id.name == "Export3DPackage" { "3D" } else { "2D" };
                        for p in paths {
                            let payload = serde_json::json!({
                                "engine": "Encheament Engine",
                                "export_kind": export_kind,
                                "project_name": self.project.name,
                                "region_count": self.project.regions.len(),
                                "character_count": self.project.characters.len(),
                                "item_count": self.project.items.len(),
                                "screen_count": self.project.screens.len(),
                                "asset_count": self.project.assets.len(),
                                "target_fps": CONFIGEDITOR.read().unwrap().target_fps,
                                "tick_ms": CONFIGEDITOR.read().unwrap().game_tick_ms,
                                "grid_size": CONFIGEDITOR.read().unwrap().grid_size,
                                "theme": self.theme_preset,
                            });
                            match serde_json::to_string_pretty(&payload) {
                                Ok(text) => {
                                    if std::fs::write(&p, text).is_ok() {
                                        ctx.ui.send(TheEvent::SetStatusText(
                                            TheId::empty(),
                                            format!("Exported {} package manifest to {:?}", export_kind, p),
                                        ));
                                    } else {
                                        ctx.ui.send(TheEvent::SetStatusText(
                                            TheId::empty(),
                                            format!("Failed to export {} package to {:?}", export_kind, p),
                                        ));
                                    }
                                }
                                Err(err) => {
                                    ctx.ui.send(TheEvent::SetStatusText(
                                        TheId::empty(),
                                        format!("Failed to create {} export payload: {}", export_kind, err),
                                    ));
                                }
                            }
                        }
                    } else
                    // Open
                    if id.name == "Open" {
                        for p in paths {
                            self.project_path = Some(p.clone());
                            self.update_counter = 0;
                            self.sidebar.startup = true;

                            // ctx.ui.set_disabled("Save");
                            // ctx.ui.set_disabled("Save As");
                            ctx.ui.set_disabled("Undo");
                            ctx.ui.set_disabled("Redo");
                            *UNDOMANAGER.write().unwrap() = UndoManager::default();

                            // let contents =
                            //     std::fs::read_to_string(p.clone()).unwrap_or("".to_string());
                            // // if let Ok(contents) = std::fs::read(p) {
                            // let pr: Result<Project, serde_json::Error> =
                            //     serde_json::from_str(&contents);
                            // println!("{:?}", pr.err());
                            if let Ok(contents) = std::fs::read_to_string(p) {
                                if let Ok(project) = serde_json::from_str(&contents) {
                                    self.project = project;
                                    self.project.palette.current_index = 0;

                                    insert_content_into_maps(&mut self.project);

                                    // Rename and remove legacy attributes
                                    for r in &mut self.project.regions {
                                        for s in &mut r.map.sectors {
                                            if let Some(floor) = s.properties.get("floor_source") {
                                                s.properties.set("source", floor.clone());
                                            }

                                            if s.properties.contains("rect_rendering") {
                                                s.properties.set("rect", Value::Bool(true));
                                            }

                                            s.properties.remove("floor_source");
                                            s.properties.remove("rect_rendering");
                                            s.properties.remove("ceiling_source");
                                        }
                                    }

                                    // Map names of characters to instances
                                    let mut hash = FxHashMap::default();
                                    for c in &self.project.characters {
                                        hash.insert(c.0, c.1.name.clone());
                                    }
                                    for r in &mut self.project.regions {
                                        for c in &mut r.characters {
                                            if let Some(n) = hash.get(&c.1.character_id) {
                                                c.1.name = n.clone();
                                            }
                                        }
                                    }

                                    // Map names of items to instances
                                    let mut hash = FxHashMap::default();
                                    for c in &self.project.items {
                                        hash.insert(c.0, c.1.name.clone());
                                    }

                                    // Apply names and sanitize map and its profiles
                                    for r in &mut self.project.regions {
                                        for c in &mut r.items {
                                            if let Some(n) = hash.get(&c.1.item_id) {
                                                c.1.name = n.clone();
                                            }
                                        }
                                        for (_, p) in &mut r.map.profiles {
                                            p.sanitize();
                                        }
                                        r.map.sanitize();
                                    }

                                    // Sanitize screens
                                    for (_, screen) in &mut self.project.screens {
                                        screen.map.sanitize();
                                    }

                                    // Convert old tile refs to new tiles
                                    if self.project.tiles.is_empty() {
                                        let tiles = self.project.extract_tiles();

                                        for (id, t) in tiles.iter() {
                                            let mut texture_array: Vec<Texture> = vec![];
                                            for b in &t.buffer {
                                                let mut texture = Texture::new(
                                                    b.pixels().to_vec(),
                                                    b.dim().width as usize,
                                                    b.dim().height as usize,
                                                );
                                                texture.generate_normals(true);
                                                texture_array.push(texture);
                                            }
                                            let tile = rusterix::Tile {
                                                id: t.id,
                                                role: rusterix::TileRole::from_index(t.role),
                                                textures: texture_array.clone(),
                                                module: None,
                                                blocking: t.blocking,
                                                scale: t.scale,
                                                tags: t.name.clone(),
                                            };
                                            self.project.tiles.insert(*id, tile);
                                        }
                                    }

                                    // Generate all tile normals
                                    for (_, tile) in self.project.tiles.iter_mut() {
                                        for texture in &mut tile.textures {
                                            texture.generate_normals(true);
                                        }
                                    }

                                    // Recompile character visual codes if scripts have Python code
                                    for (_, character) in self.project.characters.iter_mut() {
                                        if character.source.starts_with("class") {
                                            character.source = character.module.build(false);
                                            character.source_debug = character.module.build(true);
                                        }
                                    }

                                    // Recompile entity visual codes if scripts have Python code
                                    for (_, item) in self.project.items.iter_mut() {
                                        if item.source.starts_with("class") {
                                            item.source = item.module.build(false);
                                            item.source_debug = item.module.build(true);
                                        }
                                    }

                                    // Set the project time to the server time slider widget
                                    if let Some(widget) = ui.get_widget("Server Time Slider") {
                                        widget.set_value(TheValue::Time(self.project.time));
                                    }

                                    // Set the server time to the client (and if running to the server)
                                    {
                                        let mut rusterix = RUSTERIX.write().unwrap();
                                        rusterix.client.set_server_time(self.project.time);
                                        rusterix.client.global = self.project.render_graph.clone();
                                        if rusterix.server.state == rusterix::ServerState::Running {
                                            if let Some(map) =
                                                self.project.get_map(&self.server_ctx)
                                            {
                                                rusterix
                                                    .server
                                                    .set_time(&map.id, self.project.time);
                                            }
                                        }
                                    }

                                    self.server_ctx.clear();
                                    if let Some(first) = self.project.regions.first() {
                                        self.server_ctx.curr_region = first.id;
                                    }

                                    self.load_workspace_settings_from_project_config();
                                    self.apply_workspace_settings_to_ui(ui, ctx);

                                    self.sidebar.load_from_project(
                                        ui,
                                        ctx,
                                        &mut self.server_ctx,
                                        &mut self.project,
                                    );
                                    self.mapeditor.load_from_project(ui, ctx, &self.project);
                                    update_server_icons = true;
                                    redraw = true;

                                    // Set palette and textures
                                    *PALETTE.write().unwrap() = self.project.palette.clone();

                                    SCENEMANAGER
                                        .write()
                                        .unwrap()
                                        .set_palette(self.project.palette.clone());

                                    ctx.ui.send(TheEvent::SetStatusText(
                                        TheId::empty(),
                                        "Project loaded successfully.".to_string(),
                                    ));
                                }
                            }
                        }
                    } else if id.name == "Save As" {
                        for p in paths {
                            let json = serde_json::to_string(&self.project);
                            if let Ok(json) = json {
                                if std::fs::write(p.clone(), json).is_ok() {
                                    self.project_path = Some(p);
                                    ctx.ui.send(TheEvent::SetStatusText(
                                        TheId::empty(),
                                        "Project saved successfully.".to_string(),
                                    ))
                                } else {
                                    ctx.ui.send(TheEvent::SetStatusText(
                                        TheId::empty(),
                                        "Unable to save project!".to_string(),
                                    ))
                                }
                            }
                        }
                    }
                }
                TheEvent::StateChanged(id, state) => {
                    let is_direct_action =
                        state == TheWidgetState::Clicked || state == TheWidgetState::Selected;
                    if is_direct_action {
                        if id.name == "TownGenerateBtn" {
                            let seed = if self.towngen_last_seed == 0 {
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .map(|d| d.as_secs())
                                    .unwrap_or(1)
                            } else {
                                self.towngen_last_seed
                            };
                            self.generate_town_system_data_with_seed(ui, ctx, seed);
                            redraw = true;
                        } else if id.name == "FantasyWorldGenerateBtn" {
                            self.generate_fantasy_world_system_data(ui, ctx);
                            redraw = true;
                        } else if id.name == "TownRegenerateBtn" {
                            let seed = if self.towngen_last_seed == 0 {
                                std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .map(|d| d.as_secs())
                                    .unwrap_or(1)
                            } else {
                                self.towngen_last_seed
                            };
                            self.generate_town_system_data_with_seed(ui, ctx, seed);
                            redraw = true;
                        } else if id.name == "TownBakeBtn" {
                            self.bake_generated_town_to_current_map(ui, ctx);
                            redraw = true;
                        } else if id.name == "TownExportBtn" {
                            ctx.ui.save_file_requester(
                                TheId::named_with_id("ExportTownJson", Uuid::new_v4()),
                                "Export Town Data".into(),
                                TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                            );
                        } else if id.name == "TownImportBtn" {
                            ctx.ui.open_file_requester(
                                TheId::named_with_id("ImportTownJson", Uuid::new_v4()),
                                "Import Town Data".into(),
                                TheFileExtension::new("JSON".into(), vec!["json".to_string()]),
                            );
                        } else if id.name == "TownAutoQuestBtn" {
                            let n = self.generate_quests_from_last_town();
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                format!("Generated {} MMO quests from town districts.", n),
                            ));
                        } else if id.name == "TownSpawnPoiBtn" {
                            let n = self.spawn_pois_from_last_town();
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                format!("Spawned {} town POI NPCs/entities.", n),
                            ));
                            redraw = true;
                        } else if id.name == "MmorpgSimTickBtn" {
                            let msg = self.run_mmorpg_sim_tick();
                            ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                        } else if id.name == "MmorpgGenerateBtn" {
                            self.generate_rpg_mmorpg_system_data(ui, ctx);
                            redraw = true;
                        } else if id.name == "MmorpgSimCombatBtn" {
                            let msg = self.run_mmorpg_sim_combat();
                            ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                        } else if id.name == "MmorpgSimLootBtn" {
                            let msg = self.run_mmorpg_sim_loot();
                            ctx.ui.send(TheEvent::SetStatusText(TheId::empty(), msg));
                        } else if id.name == "LeftGroupAction::ExpandAll" {
                            self.left_group_modes_expanded = true;
                            self.left_group_2d_expanded = true;
                            self.left_group_3d_expanded = true;
                            self.left_group_editor_expanded = true;
                            self.rebuild_left_tool_name_layout(ui, ctx);
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "LeftGroupAction::CollapseAll" {
                            self.left_group_modes_expanded = false;
                            self.left_group_2d_expanded = false;
                            self.left_group_3d_expanded = false;
                            self.left_group_editor_expanded = false;
                            self.rebuild_left_tool_name_layout(ui, ctx);
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if let Some(group) = id.name.strip_prefix("LeftGroupToggle::") {
                            match group {
                                "Modes" => {
                                    self.left_group_modes_expanded = !self.left_group_modes_expanded
                                }
                                "2D" => self.left_group_2d_expanded = !self.left_group_2d_expanded,
                                "3D" => self.left_group_3d_expanded = !self.left_group_3d_expanded,
                                "Editor" => {
                                    self.left_group_editor_expanded =
                                        !self.left_group_editor_expanded
                                }
                                _ => {}
                            }
                            self.rebuild_left_tool_name_layout(ui, ctx);
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if let Some(tool_name) = id
                            .name
                            .strip_prefix("TopTool::")
                            .or_else(|| id.name.strip_prefix("LeftTool::"))
                            .or_else(|| id.name.strip_prefix("LeftMode::"))
                            .or_else(|| id.name.strip_prefix("RightTool::"))
                        {
                            ctx.ui.send(TheEvent::Custom(
                                TheId::named("Set Tool"),
                                TheValue::Text(tool_name.to_string()),
                            ));
                            redraw = true;
                        } else if let Some(menu_id) = id.name.strip_prefix("LeftAction::") {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("LeftToolbarAction"),
                                TheId::named(menu_id),
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiOpen" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Open"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiSave" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Save"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiUndo" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Undo"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiRedo" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Redo"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "Undo" {
                            UNDOMANAGER.write().unwrap().undo(
                                &mut self.server_ctx,
                                &mut self.project,
                                ui,
                                ctx,
                            );
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "Undo command applied.".to_string(),
                            ));
                            redraw = true;
                        } else if id.name == "Redo" {
                            UNDOMANAGER.write().unwrap().redo(
                                &mut self.server_ctx,
                                &mut self.project,
                                ui,
                                ctx,
                            );
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "Redo command applied.".to_string(),
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiPlay" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Play"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiPause" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Pause"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiStop" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Stop"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "QuickGuiHelp" {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Help"),
                                TheWidgetState::Clicked,
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickLayouts" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickLayouts"),
                                TheId::named("MenuLayouts::Unreal"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickDebugToggle" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickDebugToggle"),
                                TheId::named("MenuDebug::ToggleRuntime"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickDebugTick" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickDebugTick"),
                                TheId::named("MenuDebug::SimTick"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickSourceSubmit" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickSourceSubmit"),
                                TheId::named("MenuSource::SubmitContent"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickSourceSync" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickSourceSync"),
                                TheId::named("MenuSource::Sync"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickModeSelect" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickModeSelect"),
                                TheId::named("MenuMode::Select"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickModeLandscape" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickModeLandscape"),
                                TheId::named("MenuMode::Landscape"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickPlatformWindows" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickPlatformWindows"),
                                TheId::named("MenuPlatforms::PackageWindows"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickPlatformWeb" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickPlatformWeb"),
                                TheId::named("MenuPlatforms::PackageWeb"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickMapForgeOpen" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickMapForgeOpen"),
                                TheId::named("MenuMapForge::OpenEditor"),
                            ));
                            redraw = true;
                        } else if id.name == "TopQuickMapForgeGenerateTown" {
                            ctx.ui.send(TheEvent::ContextMenuSelected(
                                TheId::named("TopQuickMapForgeGenerateTown"),
                                TheId::named("MenuMapForge::GenerateTown"),
                            ));
                            redraw = true;
                        } else if id.name == "QuickWindowContentBrowser" {
                            self.show_ide_panel_dialog(
                                ui,
                                ctx,
                                crate::ide_panels::IdePanelKind::ContentBrowser,
                            );
                            redraw = true;
                        } else if id.name == "QuickWindowOutliner" {
                            self.show_ide_panel_dialog(
                                ui,
                                ctx,
                                crate::ide_panels::IdePanelKind::WorldOutliner,
                            );
                            redraw = true;
                        } else if id.name == "QuickWindowDetails" {
                            self.show_ide_panel_dialog(
                                ui,
                                ctx,
                                crate::ide_panels::IdePanelKind::Details,
                            );
                            redraw = true;
                        } else if id.name == "QuickWindowLog" {
                            self.show_ide_panel_dialog(
                                ui,
                                ctx,
                                crate::ide_panels::IdePanelKind::OutputLog,
                            );
                            redraw = true;
                        } else if id.name == "QuickWindowBlueprint" {
                            self.show_ide_panel_dialog(
                                ui,
                                ctx,
                                crate::ide_panels::IdePanelKind::Blueprint,
                            );
                            redraw = true;
                        } else if id.name == "QuickMmorpgBuilder" {
                            self.show_rpg_mmorpg_builder_dialog(ui, ctx);
                            redraw = true;
                        } else if id.name == "QuickMmorpgGenerate" {
                            self.generate_rpg_mmorpg_system_data(ui, ctx);
                            redraw = true;
                        } else if id.name == "QuickThemeDark" {
                            self.theme_preset = "Dark".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(0));
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "QuickThemeLight" {
                            self.theme_preset = "Light".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(1));
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "QuickThemeSlate" {
                            self.theme_preset = "Slate".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(2));
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "QuickOptSnap" {
                            self.option_snap_to_grid = !self.option_snap_to_grid;
                            self.server_ctx.snap_to_grid = self.option_snap_to_grid;
                            ui.set_widget_value(
                                "OptionSnapCB",
                                ctx,
                                TheValue::Bool(self.option_snap_to_grid),
                            );
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "QuickOptGrid" {
                            self.option_show_grid = !self.option_show_grid;
                            ui.set_widget_value(
                                "OptionGridCB",
                                ctx,
                                TheValue::Bool(self.option_show_grid),
                            );
                            workspace_prefs_dirty = true;
                            redraw = true;
                        } else if id.name == "QuickOptGizmos" {
                            self.option_show_gizmos = !self.option_show_gizmos;
                            self.server_ctx.show_editing_geometry = self.option_show_gizmos;
                            ui.set_widget_value(
                                "OptionGizmoCB",
                                ctx,
                                TheValue::Bool(self.option_show_gizmos),
                            );
                            workspace_prefs_dirty = true;
                            redraw = true;
                        }
                    }

                    if id.name == "Help" {
                        self.server_ctx.help_mode = state == TheWidgetState::Clicked;
                        if state == TheWidgetState::Clicked {
                            self.show_help_dialog(ui, ctx);
                        }
                    }
                    if id.name == "GameInput" {
                        self.server_ctx.game_input_mode = state == TheWidgetState::Clicked;
                    } else if id.name == "New" {
                        self.project_path = None;
                        self.update_counter = 0;
                        self.sidebar.startup = true;
                        self.project = Project::default();

                        if let Some(bytes) = crate::Embedded::get("starter_project.eldiron") {
                            if let Ok(project_string) = std::str::from_utf8(bytes.data.as_ref()) {
                                if let Ok(project) =
                                    serde_json::from_str(&project_string.to_string())
                                {
                                    self.project = project;
                                }
                            }
                        }

                        // ctx.ui.set_disabled("Save");
                        // ctx.ui.set_disabled("Save As");
                        ctx.ui.set_disabled("Undo");
                        ctx.ui.set_disabled("Redo");
                        *UNDOMANAGER.write().unwrap() = UndoManager::default();

                        insert_content_into_maps(&mut self.project);

                        // Set the project time to the server time slider widget
                        if let Some(widget) = ui.get_widget("Server Time Slider") {
                            widget.set_value(TheValue::Time(self.project.time));
                        }

                        // Set the server time to the client (and if running to the server)
                        {
                            let mut rusterix = RUSTERIX.write().unwrap();
                            rusterix.client.set_server_time(self.project.time);
                            if rusterix.server.state == rusterix::ServerState::Running {
                                if let Some(map) = self.project.get_map(&self.server_ctx) {
                                    rusterix.server.set_time(&map.id, self.project.time);
                                }
                            }
                        }

                        self.server_ctx.clear();
                        self.sidebar.load_from_project(
                            ui,
                            ctx,
                            &mut self.server_ctx,
                            &mut self.project,
                        );
                        self.mapeditor.load_from_project(ui, ctx, &self.project);
                        update_server_icons = true;
                        redraw = true;

                        // Set palette and textures
                        *PALETTE.write().unwrap() = self.project.palette.clone();

                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "New project successfully initialized.".to_string(),
                        ));
                    } else if id.name == "Logo" {
                        redraw = true;
                    } else if id.name == "BlueprintEditor" || id.name == "OpenBlueprintEditor" {
                        let launch = crate::features::launch_blueprint_editor();
                        self.show_ide_panel_dialog(
                            ui,
                            ctx,
                            crate::ide_panels::IdePanelKind::Blueprint,
                        );
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            launch.status_line,
                        ));
                        _ = open::that(launch.docs_url);
                        ctx.ui.set_widget_state(
                            "BlueprintEditor".to_string(),
                            TheWidgetState::None,
                        );
                        ctx.ui.clear_hover();
                        redraw = true;
                    } else if id.name == "Patreon" {
                        _ = open::that("https://www.patreon.com/eldiron");
                        ctx.ui
                            .set_widget_state("Patreon".to_string(), TheWidgetState::None);
                        ctx.ui.clear_hover();
                        redraw = true;
                    } else if id.name == "Update" {
                        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
                        {
                            let updater = self.self_updater.lock().unwrap();

                            if updater.has_newer_release() {
                                self.self_update_tx
                                    .send(SelfUpdateEvent::UpdateConfirm(
                                        updater.latest_release().cloned().unwrap(),
                                    ))
                                    .unwrap();
                            } else {
                                if let Some(statusbar) = ui.get_widget("Statusbar") {
                                    statusbar
                                        .as_statusbar()
                                        .unwrap()
                                        .set_text(fl!("info_update_check"));
                                }

                                let updater = Arc::clone(&self.self_updater);
                                let tx = self.self_update_tx.clone();

                                thread::spawn(move || {
                                    let mut updater = updater.lock().unwrap();

                                    match updater.fetch_release_list() {
                                        Ok(_) => {
                                            if updater.has_newer_release() {
                                                tx.send(SelfUpdateEvent::UpdateConfirm(
                                                    updater.latest_release().cloned().unwrap(),
                                                ))
                                                .unwrap();
                                            } else {
                                                tx.send(SelfUpdateEvent::AlreadyUpToDate).unwrap();
                                            }
                                        }
                                        Err(err) => {
                                            tx.send(SelfUpdateEvent::UpdateError(err.to_string()))
                                                .unwrap();
                                        }
                                    }
                                });
                            }

                            ctx.ui
                                .set_widget_state("Update".to_string(), TheWidgetState::None);
                            ctx.ui.clear_hover();
                            redraw = true;
                        }
                    } else if id.name == "Open" {
                        ctx.ui.open_file_requester(
                            TheId::named_with_id(id.name.as_str(), Uuid::new_v4()),
                            "Open".into(),
                            TheFileExtension::new("Eldiron".into(), vec!["eldiron".to_string()]),
                        );
                        ctx.ui
                            .set_widget_state("Open".to_string(), TheWidgetState::None);
                        ctx.ui.clear_hover();
                        redraw = true;
                    } else if id.name == "Save" {
                        if let Some(path) = &self.project_path {
                            let mut success = false;
                            // if let Ok(output) = postcard::to_allocvec(&self.project) {
                            if let Ok(output) = serde_json::to_string(&self.project) {
                                if std::fs::write(path, output).is_ok() {
                                    ctx.ui.send(TheEvent::SetStatusText(
                                        TheId::empty(),
                                        "Project saved successfully.".to_string(),
                                    ));
                                    success = true;
                                }
                            }

                            if !success {
                                ctx.ui.send(TheEvent::SetStatusText(
                                    TheId::empty(),
                                    "Unable to save project!".to_string(),
                                ))
                            }
                        } else {
                            ctx.ui.send(TheEvent::StateChanged(
                                TheId::named("Save As"),
                                TheWidgetState::Clicked,
                            ));
                            ctx.ui
                                .set_widget_state("Save".to_string(), TheWidgetState::None);
                        }
                    } else if id.name == "Save As" {
                        ctx.ui.save_file_requester(
                            TheId::named_with_id(id.name.as_str(), Uuid::new_v4()),
                            "Save".into(),
                            TheFileExtension::new("Eldiron".into(), vec!["eldiron".to_string()]),
                        );
                        ctx.ui
                            .set_widget_state("Save As".to_string(), TheWidgetState::None);
                        ctx.ui.clear_hover();
                        redraw = true;
                    }
                    // Server
                    else if id.name == "Play" {
                        let state = RUSTERIX.read().unwrap().server.state;
                        if state == rusterix::ServerState::Paused {
                            RUSTERIX.write().unwrap().server.continue_instances();
                            update_server_icons = true;
                        } else {
                            if state == rusterix::ServerState::Off {
                                start_server(
                                    &mut RUSTERIX.write().unwrap(),
                                    &mut self.project,
                                    true,
                                );
                                let commands =
                                    setup_client(&mut RUSTERIX.write().unwrap(), &mut self.project);
                                RUSTERIX
                                    .write()
                                    .unwrap()
                                    .server
                                    .process_client_commands(commands);
                                ctx.ui.send(TheEvent::SetStatusText(
                                    TheId::empty(),
                                    "Server has been started.".to_string(),
                                ));
                                // ui.set_widget_value("LogEdit", ctx, TheValue::Text(String::new()));
                                // ctx.ui.send(TheEvent::StateChanged(
                                //     TheId::named("Debug Log"),
                                //     TheWidgetState::Clicked,
                                // ));
                                RUSTERIX.write().unwrap().player_camera = PlayerCamera::D2;
                            }
                            /*
                            self.server.start();
                            self.client.reset();
                            self.client.set_project(self.project.clone());
                            self.server_ctx.clear_interactions();
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "Server has been started.".to_string(),
                            ));
                            self.sidebar.clear_debug_messages(ui, ctx);
                            */
                            update_server_icons = true;
                        }
                    } else if id.name == "Pause" {
                        let state = RUSTERIX.read().unwrap().server.state;
                        if state == rusterix::ServerState::Running {
                            RUSTERIX.write().unwrap().server.pause();
                            update_server_icons = true;
                        }
                        /*
                        if self.server.state == ServerState::Running {
                            self.server.state = ServerState::Paused;
                            ctx.ui.send(TheEvent::SetStatusText(
                                TheId::empty(),
                                "Server has been paused.".to_string(),
                            ));
                            update_server_icons = true;
                        } else if self.server.state == ServerState::Paused {
                            self.client.tick(
                                *ACTIVEEDITOR.lock().unwrap() == ActiveEditor::GameEditor,
                            );
                            let debug = self.server.tick();
                            if !debug.is_empty() {
                                self.sidebar.add_debug_messages(debug, ui, ctx);
                            }
                            let interactions = self.server.get_interactions();
                            self.server_ctx.add_interactions(interactions);
                        }*/
                    } else if id.name == "Stop" {
                        RUSTERIX.write().unwrap().server.stop();
                        RUSTERIX.write().unwrap().player_camera = PlayerCamera::D2;

                        ui.set_widget_value("InfoView", ctx, TheValue::Text("".into()));
                        /*
                        _ = self.server.set_project(self.project.clone());
                        self.server.stop();*/
                        insert_content_into_maps(&mut self.project);
                        update_server_icons = true;

                        ctx.ui.send(TheEvent::Custom(
                            TheId::named("Render SceneManager Map"),
                            TheValue::Empty,
                        ));
                    } else if id.name == "Undo" || id.name == "Redo" {
                        let mut refresh_action_ui = false;
                        if ui.focus_widget_supports_undo_redo(ctx) {
                            if id.name == "Undo" {
                                ui.undo(ctx);
                            } else {
                                ui.redo(ctx);
                            }
                        } else if DOCKMANAGER.read().unwrap().current_dock_supports_undo() {
                            if id.name == "Undo" {
                                DOCKMANAGER.write().unwrap().undo(
                                    ui,
                                    ctx,
                                    &mut self.project,
                                    &mut self.server_ctx,
                                );
                            } else {
                                DOCKMANAGER.write().unwrap().redo(
                                    ui,
                                    ctx,
                                    &mut self.project,
                                    &mut self.server_ctx,
                                );
                            }
                            refresh_action_ui = true;
                        } else {
                            let mut manager = UNDOMANAGER.write().unwrap();

                            if id.name == "Undo" {
                                manager.undo(&mut self.server_ctx, &mut self.project, ui, ctx);
                            } else {
                                manager.redo(&mut self.server_ctx, &mut self.project, ui, ctx);
                            }
                            refresh_action_ui = true;
                        }

                        // Keep action list and TOML params in sync only when project/dock state changed.
                        if refresh_action_ui {
                            ctx.ui.send(TheEvent::Custom(
                                TheId::named("Update Action List"),
                                TheValue::Empty,
                            ));
                            ctx.ui.send(TheEvent::Custom(
                                TheId::named("Update Action Parameters"),
                                TheValue::Empty,
                            ));
                        }
                    } else if id.name == "Cut" {
                        if ui.focus_widget_supports_clipboard(ctx) {
                            // Widget specific
                            ui.cut(ctx);
                        } else {
                            // Global
                            ctx.ui.send(TheEvent::Cut);
                        }
                    } else if id.name == "Copy" {
                        if ui.focus_widget_supports_clipboard(ctx) {
                            // Widget specific
                            ui.copy(ctx);
                        } else {
                            // Global
                            ctx.ui.send(TheEvent::Copy);
                        }
                    } else if id.name == "Paste" {
                        if ui.focus_widget_supports_clipboard(ctx) {
                            // Widget specific
                            ui.paste(ctx);
                        } else {
                            // Global
                            if let Some(value) = &ctx.ui.clipboard {
                                ctx.ui.send(TheEvent::Paste(
                                    value.clone(),
                                    ctx.ui.clipboard_app_type.clone(),
                                ));
                            } else {
                                ctx.ui.send(TheEvent::Paste(
                                    TheValue::Empty,
                                    ctx.ui.clipboard_app_type.clone(),
                                ));
                            }
                        }
                    }
                }
                TheEvent::IndexChanged(id, index) => {
                    if id.name == "ThemePresetDropdown" {
                        self.theme_preset = match index {
                            1 => "Light".to_string(),
                            2 => "Slate".to_string(),
                            _ => "Dark".to_string(),
                        };
                        self.apply_theme_preset(ui, ctx);
                        workspace_prefs_dirty = true;
                    } else if id.name == "TownPresetDropdown" {
                        self.towngen_preset = match index {
                            0 => "Small Town".to_string(),
                            1 => "Large Town".to_string(),
                            3 => "Large City".to_string(),
                            _ => "Small City".to_string(),
                        };
                        workspace_prefs_dirty = true;
                    }
                }
                TheEvent::ValueChanged(id, value) => {
                    if id.name == "Server Time Slider" {
                        if let TheValue::Time(time) = value {
                            self.project.time = time;
                            let mut rusterix = RUSTERIX.write().unwrap();
                            rusterix.client.set_server_time(time);

                            if rusterix.server.state == rusterix::ServerState::Running {
                                if let Some(map) = self.project.get_map(&self.server_ctx) {
                                    rusterix.server.set_time(&map.id, time);
                                }
                            }
                        }
                    } else if id.name == "OptionSnapCB" {
                        if let TheValue::Bool(v) = value {
                            self.option_snap_to_grid = v;
                            self.server_ctx.snap_to_grid = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OptionGridCB" {
                        if let TheValue::Bool(v) = value {
                            self.option_show_grid = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OptionGizmoCB" {
                        if let TheValue::Bool(v) = value {
                            self.option_show_gizmos = v;
                            self.server_ctx.show_editing_geometry = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OptionLeftToolbarCB" {
                        if let TheValue::Bool(v) = value {
                            self.show_left_toolbar = v;
                            self.apply_toolbar_visibility(ui, ctx);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OptionRightToolbarCB" {
                        if let TheValue::Bool(v) = value {
                            self.show_right_toolbar = v;
                            self.apply_toolbar_visibility(ui, ctx);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OptionTargetFpsEdit" {
                        if let Some(v) = value.to_i32() {
                            CONFIGEDITOR.write().unwrap().target_fps = v.clamp(1, 120);
                        }
                    } else if id.name == "OptionTickMsEdit" {
                        if let Some(v) = value.to_i32() {
                            CONFIGEDITOR.write().unwrap().game_tick_ms = v.clamp(10, 2000);
                        }
                    } else if id.name == "OptionGridSizeEdit" {
                        if let Some(v) = value.to_i32() {
                            CONFIGEDITOR.write().unwrap().grid_size = v.clamp(4, 256);
                        }
                    } else if id.name == "TownSeedEdit" {
                        if let Some(v) = value.to_i32() {
                            self.towngen_last_seed = v.max(1) as u64;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "TownRiverCB" {
                        if let TheValue::Bool(v) = value {
                            self.towngen_has_river = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "TownWallsCB" {
                        if let TheValue::Bool(v) = value {
                            self.towngen_has_walls = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "TownAutoBakeCB" {
                        if let TheValue::Bool(v) = value {
                            self.towngen_auto_bake = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OverlayTownDistrictsCB" {
                        if let TheValue::Bool(v) = value {
                            self.overlay_show_town_districts = v;
                            self.apply_town_overlay_visibility();
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OverlayTownRoadsCB" {
                        if let TheValue::Bool(v) = value {
                            self.overlay_show_town_roads = v;
                            self.apply_town_overlay_visibility();
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "OverlayTownLandmarksCB" {
                        if let TheValue::Bool(v) = value {
                            self.overlay_show_town_landmarks = v;
                            self.apply_town_overlay_visibility();
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyWorldNameEdit" {
                        if let TheValue::Text(v) = value {
                            self.fantasy_world_name = v.clone();
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyWorldSeedEdit" {
                        if let Some(v) = value.to_i32() {
                            self.fantasy_world_seed = v.max(0) as u64;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyContinentCountEdit" {
                        if let Some(v) = value.to_i32() {
                            self.fantasy_continent_count = v.clamp(1, 12);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyCountriesPerContinentEdit" {
                        if let Some(v) = value.to_i32() {
                            self.fantasy_countries_per_continent = v.clamp(1, 24);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyTownsPerCountryEdit" {
                        if let Some(v) = value.to_i32() {
                            self.fantasy_towns_per_country = v.clamp(1, 20);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "FantasyHasIslandsCB" {
                        if let TheValue::Bool(v) = value {
                            self.fantasy_has_islands = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgXpRateEdit" {
                        if let Some(v) = value.to_f32() {
                            self.mmorpg_xp_rate = v.clamp(0.1, 5.0);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgLootRateEdit" {
                        if let Some(v) = value.to_f32() {
                            self.mmorpg_loot_rate = v.clamp(0.1, 5.0);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgEventRateEdit" {
                        if let Some(v) = value.to_f32() {
                            self.mmorpg_event_rate = v.clamp(0.1, 5.0);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgWorldNameEdit" {
                        if let TheValue::Text(v) = value {
                            self.mmorpg_world_name = v.clone();
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgMaxPlayersEdit" {
                        if let Some(v) = value.to_i32() {
                            self.mmorpg_max_players = v.clamp(10, 100_000);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgStartingLevelEdit" {
                        if let Some(v) = value.to_i32() {
                            self.mmorpg_starting_level = v.clamp(1, 99);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgRaceCountEdit" {
                        if let Some(v) = value.to_i32() {
                            self.mmorpg_race_count = v.clamp(1, 12);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgQuestCountEdit" {
                        if let Some(v) = value.to_i32() {
                            self.mmorpg_quest_count = v.clamp(1, 64);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgSkillTierCountEdit" {
                        if let Some(v) = value.to_i32() {
                            self.mmorpg_skill_tier_count = v.clamp(1, 6);
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgClassWarriorCB" {
                        if let TheValue::Bool(v) = value {
                            self.mmorpg_include_warrior = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgClassRangerCB" {
                        if let TheValue::Bool(v) = value {
                            self.mmorpg_include_ranger = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgClassMageCB" {
                        if let TheValue::Bool(v) = value {
                            self.mmorpg_include_mage = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgClassClericCB" {
                        if let TheValue::Bool(v) = value {
                            self.mmorpg_include_cleric = v;
                            workspace_prefs_dirty = true;
                        }
                    } else if id.name == "MmorpgClassRogueCB" {
                        if let TheValue::Bool(v) = value {
                            self.mmorpg_include_rogue = v;
                            workspace_prefs_dirty = true;
                        }
                    }
                }
                _ => {}
            }
        }

        if workspace_prefs_dirty {
            self.persist_workspace_settings_to_project_config();
        }

        #[cfg(all(not(target_arch = "wasm32"), feature = "self-update"))]
        while let Ok(event) = self.self_update_rx.try_recv() {
            match event {
                SelfUpdateEvent::AlreadyUpToDate => {
                    let text = str!("Eldiron is already up-to-date.");
                    let uuid = Uuid::new_v4();

                    let width = 300;
                    let height = 100;

                    let mut canvas = TheCanvas::new();
                    canvas.limiter_mut().set_max_size(Vec2::new(width, height));

                    let mut hlayout: TheHLayout = TheHLayout::new(TheId::empty());
                    hlayout.limiter_mut().set_max_width(width);

                    let mut text_widget = TheText::new(TheId::named_with_id("Dialog Value", uuid));
                    text_widget.set_text(text.to_string());
                    text_widget.limiter_mut().set_max_width(200);
                    hlayout.add_widget(Box::new(text_widget));

                    canvas.set_layout(hlayout);

                    ui.show_dialog(
                        "Eldiron Up-to-Date",
                        canvas,
                        vec![TheDialogButtonRole::Accept],
                        ctx,
                    );
                }
                SelfUpdateEvent::UpdateCompleted(release) => {
                    if let Some(statusbar) = ui.get_widget("Statusbar") {
                        statusbar
                            .as_statusbar()
                            .unwrap()
                            .set_text(format!(
                                "Updated to version {}. Please restart the application to enjoy the new features.",
                                release.version
                            ));
                    }
                }
                SelfUpdateEvent::UpdateConfirm(release) => {
                    let text = &format!("Update to version {}?", release.version);
                    let uuid = Uuid::new_v4();

                    let width = 300;
                    let height = 100;

                    let mut canvas = TheCanvas::new();
                    canvas.limiter_mut().set_max_size(Vec2::new(width, height));

                    let mut hlayout: TheHLayout = TheHLayout::new(TheId::empty());
                    hlayout.limiter_mut().set_max_width(width);

                    let mut text_widget = TheText::new(TheId::named_with_id("Dialog Value", uuid));
                    text_widget.set_text(text.to_string());
                    text_widget.limiter_mut().set_max_width(200);
                    hlayout.add_widget(Box::new(text_widget));

                    canvas.set_layout(hlayout);

                    ui.show_dialog(
                        "Update Eldiron",
                        canvas,
                        vec![TheDialogButtonRole::Accept, TheDialogButtonRole::Reject],
                        ctx,
                    );
                }
                SelfUpdateEvent::UpdateError(err) => {
                    if let Some(statusbar) = ui.get_widget("Statusbar") {
                        statusbar
                            .as_statusbar()
                            .unwrap()
                            .set_text(format!("Failed to update Eldiron: {err}"));
                    }
                }
                SelfUpdateEvent::UpdateStart(release) => {
                    if let Some(statusbar) = ui.get_widget("Statusbar") {
                        statusbar
                            .as_statusbar()
                            .unwrap()
                            .set_text(format!("Updating to version {}...", release.version));
                    }
                }
            }
        }

        if update_server_icons {
            self.update_server_state_icons(ui);
            redraw = true;
        }
        self.update_counter += 1;
        if self.update_counter > 2 {
            self.sidebar.startup = false;
        }
        redraw
    }

    /// Returns true if there are changes
    fn has_changes(&self) -> bool {
	UNDOMANAGER.read().unwrap().has_undo() || DOCKMANAGER.read().unwrap().has_dock_changes()
    }
}

pub trait EldironEditor {
    // ...existing trait methods...
    fn update_server_state_icons(&mut self, ui: &mut TheUI);
}

impl EldironEditor for Editor {
    fn update_server_state_icons(&mut self, ui: &mut TheUI) {
        let rusterix = RUSTERIX.read().unwrap();
        if rusterix.server.state == rusterix::ServerState::Running {
            if let Some(button) = ui.get_widget("Play") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play-fill".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Pause") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play-pause".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Stop") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("stop".to_string());
                }
            }
        } else if rusterix.server.state == rusterix::ServerState::Paused {
            if let Some(button) = ui.get_widget("Play") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Pause") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play-pause-fill".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Stop") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("stop".to_string());
                }
            }
        } else if rusterix.server.state == rusterix::ServerState::Off {
            if let Some(button) = ui.get_widget("Play") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Pause") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("play-pause".to_string());
                }
            }
            if let Some(button) = ui.get_widget("Stop") {
                if let Some(button) = button.as_menubar_button() {
                    button.set_icon_name("stop-fill".to_string());
                }
            }
        }
    }
}
