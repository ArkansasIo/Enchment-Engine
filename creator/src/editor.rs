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
}


impl Editor {
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
        if let Some(left_layout) = ui.get_layout("Tool List Layout") {
            left_layout
                .limiter_mut()
                .set_max_width(if self.show_left_toolbar { 51 } else { 0 });
            left_layout.relayout(ctx);
        }
        if let Some(right_layout) = ui.get_layout("Right Tool Layout") {
            right_layout
                .limiter_mut()
                .set_max_width(if self.show_right_toolbar { 51 } else { 0 });
            right_layout.relayout(ctx);
        }
    }

    fn apply_theme_preset(&self, ui: &mut TheUI, ctx: &mut TheContext) {
        let top_color = match self.theme_preset.as_str() {
            "Light" => Some(TheThemeColors::TextLayoutBackground),
            "Slate" => Some(TheThemeColors::DefaultWidgetDarkBackground),
            _ => None,
        };
        let side_color = match self.theme_preset.as_str() {
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
        if let Some(layout) = ui.get_layout("Tool List Layout") {
            layout.set_background_color(side_color);
            layout.relayout(ctx);
        }
        if let Some(layout) = ui.get_layout("Right Tool Layout") {
            layout.set_background_color(side_color);
            layout.relayout(ctx);
        }
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
        time_slider.limiter_mut().set_max_width(400);
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

        let mut tools_2d = TheContextMenu::named("2D".to_string());
        tools_2d.add(TheContextMenuItem::new("Selection".to_string(), TheId::named("MenuTool::Select Tool")));
        tools_2d.add(TheContextMenuItem::new("Vertex".to_string(), TheId::named("MenuTool::Vertex Tool")));
        tools_2d.add(TheContextMenuItem::new("Linedef".to_string(), TheId::named("MenuTool::Linedef Tool")));
        tools_2d.add(TheContextMenuItem::new("Sector".to_string(), TheId::named("MenuTool::Sector Tool")));

        let mut tools_3d = TheContextMenu::named("3D".to_string());
        tools_3d.add(TheContextMenuItem::new("Terrain".to_string(), TheId::named("MenuTool::World Tool")));
        tools_3d.add(TheContextMenuItem::new("Render".to_string(), TheId::named("MenuTool::Render Tool")));
        tools_3d.add(TheContextMenuItem::new("Entity".to_string(), TheId::named("MenuTool::Entity Tool")));

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
        tools_menu.add_separator();
        tools_menu.add(TheContextMenuItem::new("Code".to_string(), TheId::named("MenuTool::Code Tool")));
        tools_menu.add(TheContextMenuItem::new("Data".to_string(), TheId::named("MenuTool::Data Tool")));
        tools_menu.add(TheContextMenuItem::new("Config".to_string(), TheId::named("MenuTool::Config Tool")));
        tools_menu.add(TheContextMenuItem::new("Info".to_string(), TheId::named("MenuTool::Info Tool")));

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

        let mut help_menu = TheContextMenu::named("Help".to_string());
        help_menu.add(TheContextMenuItem::new("Docs".to_string(), TheId::named("MenuHelp::Docs")));
        help_menu.add(TheContextMenuItem::new("Examples".to_string(), TheId::named("MenuHelp::Examples")));
        help_menu.add(TheContextMenuItem::new("About".to_string(), TheId::named("MenuHelp::About")));

        main_menu.add_context_menu(file_menu);
        main_menu.add_context_menu(edit_menu);
        main_menu.add_context_menu(view_menu);
        main_menu.add_context_menu(tools_menu);
        main_menu.add_context_menu(build_menu);
        main_menu.add_context_menu(settings_menu);
        main_menu.add_context_menu(help_menu);

        menu_canvas.set_widget(main_menu);
        top_canvas.set_top(menu_canvas);

        // Top quick tool bar
        let mut top_quick_canvas = TheCanvas::new();
        top_quick_canvas.set_widget(TheToolListBar::new(TheId::named("Top Quick Tool Bar")));
        let mut top_quick_layout = TheHLayout::new(TheId::named("Top Quick Tool Layout"));
        top_quick_layout.set_background_color(None);
        top_quick_layout.set_margin(Vec4::new(10, 2, 10, 2));
        top_quick_layout.set_padding(1);
        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let quick_id = format!("TopTool::{}", tool.id().name);
                let mut button =
                    TheToolListButton::new(TheId::named(&quick_id));
                button.set_icon_name(tool.icon_name());
                button.set_status_text(&format!("Activate {}", tool.info()));
                top_quick_layout.add_widget(Box::new(button));
            }
        }
        top_quick_canvas.set_layout(top_quick_layout);
        top_canvas.set_bottom(top_quick_canvas);
        ui.canvas.set_top(top_canvas);

        // Sidebar
        self.sidebar.init_ui(ui, ctx, &mut self.server_ctx);

        // Docks
        let bottom_panels = DOCKMANAGER.write().unwrap().init(ctx);

        let mut editor_canvas: TheCanvas = TheCanvas::new();

        let mut editor_stack = TheStackLayout::new(TheId::named("Editor Stack"));
        let poly_canvas = self.mapeditor.init_ui(ui, ctx, &mut self.project);
        editor_stack.add_canvas(poly_canvas);

        // Add Dock Editors
        DOCKMANAGER
            .write()
            .unwrap()
            .add_editors_to_stack(&mut editor_stack, ctx);

        editor_canvas.set_layout(editor_stack);

        // Main V Layout
        let mut vsplitlayout = TheSharedVLayout::new(TheId::named("Shared VLayout"));
        vsplitlayout.add_canvas(editor_canvas);
        vsplitlayout.add_canvas(bottom_panels);
        vsplitlayout.set_shared_ratio(crate::DEFAULT_VLAYOUT_RATIO);
        vsplitlayout.set_mode(TheSharedVLayoutMode::Shared);

        let mut shared_canvas = TheCanvas::new();
        shared_canvas.set_layout(vsplitlayout);

        // Tool List
        let mut tool_list_canvas: TheCanvas = TheCanvas::new();

        let mut tool_list_bar_canvas = TheCanvas::new();
        tool_list_bar_canvas.set_widget(TheToolListBar::new(TheId::empty()));
        tool_list_canvas.set_top(tool_list_bar_canvas);

        let mut v_tool_list_layout = TheVLayout::new(TheId::named("Tool List Layout"));
        v_tool_list_layout.limiter_mut().set_max_width(51);
        v_tool_list_layout.set_margin(Vec4::new(2, 2, 2, 2));
        v_tool_list_layout.set_padding(1);

        TOOLLIST
            .write()
            .unwrap()
            .set_active_editor(&mut v_tool_list_layout, ctx);

        tool_list_canvas.set_layout(v_tool_list_layout);

        let mut tool_list_border_canvas = TheCanvas::new();
        let mut border_widget = TheIconView::new(TheId::empty());
        border_widget.set_border_color(Some([82, 82, 82, 255]));
        border_widget.limiter_mut().set_max_width(1);
        border_widget.limiter_mut().set_max_height(i32::MAX);
        tool_list_border_canvas.set_widget(border_widget);

        tool_list_canvas.set_right(tool_list_border_canvas);
        shared_canvas.set_left(tool_list_canvas);

        // Right side GUI/API toolbar
        let mut right_tool_canvas = TheCanvas::new();

        let mut right_tool_bar_canvas = TheCanvas::new();
        right_tool_bar_canvas.set_widget(TheToolListBar::new(TheId::named("Right Tool Bar")));
        right_tool_canvas.set_top(right_tool_bar_canvas);

        let mut right_tool_layout = TheVLayout::new(TheId::named("Right Tool Layout"));
        right_tool_layout.limiter_mut().set_max_width(51);
        right_tool_layout.set_margin(Vec4::new(2, 2, 2, 2));
        right_tool_layout.set_padding(1);

        let mut add_quick_button = |id: &str, icon: &str, status: &str| {
            let mut button = TheToolListButton::new(TheId::named(id));
            button.set_icon_name(icon.to_string());
            button.set_status_text(status);
            right_tool_layout.add_widget(Box::new(button));
        };

        add_quick_button("QuickGuiOpen", "icon_role_load", "Open project");
        add_quick_button("QuickGuiSave", "icon_role_save", "Save project");
        add_quick_button("QuickGuiUndo", "icon_role_undo", "Undo");
        add_quick_button("QuickGuiRedo", "icon_role_redo", "Redo");
        add_quick_button("QuickGuiPlay", "play", "Run game");
        add_quick_button("QuickGuiPause", "play-pause", "Pause game");
        add_quick_button("QuickGuiStop", "stop-fill", "Stop game");
        add_quick_button("QuickGuiHelp", "question-mark", "Toggle help mode");
        add_quick_button("QuickThemeDark", "dark_tabbar_selected", "Dark theme preset");
        add_quick_button("QuickThemeLight", "dark_tabbar_hover", "Light theme preset");
        add_quick_button("QuickThemeSlate", "dark_tabbar_normal", "Slate theme preset");
        add_quick_button("QuickOptSnap", "selection", "Toggle snap-to-grid");
        add_quick_button("QuickOptGrid", "square", "Toggle grid visibility");
        add_quick_button("QuickOptGizmos", "transform", "Toggle gizmos");

        if let Ok(toollist) = TOOLLIST.read() {
            for tool in &toollist.game_tools {
                let quick_id = format!("RightTool::{}", tool.id().name);
                let mut button =
                    TheToolListButton::new(TheId::named(&quick_id));
                button.set_icon_name(tool.icon_name());
                button.set_status_text(&format!("Activate {}", tool.info()));
                right_tool_layout.add_widget(Box::new(button));
            }
        }

        right_tool_canvas.set_layout(right_tool_layout);

        // Right side settings/options panel
        let mut right_settings_canvas = TheCanvas::new();
        right_settings_canvas
            .set_widget(TheTraybar::new(TheId::named("Right Settings Bar")));

        let mut right_settings_layout = TheTextLayout::new(TheId::named("Right Settings Layout"));
        right_settings_layout.set_margin(Vec4::new(4, 4, 4, 4));
        right_settings_layout.set_padding(4);
        right_settings_layout.limiter_mut().set_max_width(260);

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
        right_settings_layout.add_pair("Theme".to_string(), Box::new(theme_dropdown));

        let mut snap_cb = TheCheckButton::new(TheId::named("OptionSnapCB"));
        snap_cb.set_value(TheValue::Bool(self.option_snap_to_grid));
        right_settings_layout.add_pair("Snap".to_string(), Box::new(snap_cb));

        let mut grid_cb = TheCheckButton::new(TheId::named("OptionGridCB"));
        grid_cb.set_value(TheValue::Bool(self.option_show_grid));
        right_settings_layout.add_pair("Grid".to_string(), Box::new(grid_cb));

        let mut gizmo_cb = TheCheckButton::new(TheId::named("OptionGizmoCB"));
        gizmo_cb.set_value(TheValue::Bool(self.option_show_gizmos));
        right_settings_layout.add_pair("Gizmos".to_string(), Box::new(gizmo_cb));

        let mut left_cb = TheCheckButton::new(TheId::named("OptionLeftToolbarCB"));
        left_cb.set_value(TheValue::Bool(self.show_left_toolbar));
        right_settings_layout.add_pair("Left Bar".to_string(), Box::new(left_cb));

        let mut right_cb = TheCheckButton::new(TheId::named("OptionRightToolbarCB"));
        right_cb.set_value(TheValue::Bool(self.show_right_toolbar));
        right_settings_layout.add_pair("Right Bar".to_string(), Box::new(right_cb));

        let mut fps_edit = TheTextLineEdit::new(TheId::named("OptionTargetFpsEdit"));
        fps_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().target_fps));
        fps_edit.set_range(TheValue::RangeI32(1..=120));
        fps_edit.set_continuous(true);
        right_settings_layout.add_pair("FPS".to_string(), Box::new(fps_edit));

        let mut tick_edit = TheTextLineEdit::new(TheId::named("OptionTickMsEdit"));
        tick_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().game_tick_ms));
        tick_edit.set_range(TheValue::RangeI32(10..=2000));
        tick_edit.set_continuous(true);
        right_settings_layout.add_pair("Tick ms".to_string(), Box::new(tick_edit));

        let mut grid_size_edit = TheTextLineEdit::new(TheId::named("OptionGridSizeEdit"));
        grid_size_edit.set_value(TheValue::Int(CONFIGEDITOR.read().unwrap().grid_size));
        grid_size_edit.set_range(TheValue::RangeI32(4..=256));
        grid_size_edit.set_continuous(true);
        right_settings_layout.add_pair("Grid Size".to_string(), Box::new(grid_size_edit));

        right_settings_canvas.set_layout(right_settings_layout);
        right_tool_canvas.set_bottom(right_settings_canvas);

        let mut right_tool_border_canvas = TheCanvas::new();
        let mut right_border_widget = TheIconView::new(TheId::empty());
        right_border_widget.set_border_color(Some([82, 82, 82, 255]));
        right_border_widget.limiter_mut().set_max_width(1);
        right_border_widget.limiter_mut().set_max_height(i32::MAX);
        right_tool_border_canvas.set_widget(right_border_widget);
        right_tool_canvas.set_left(right_tool_border_canvas);

        shared_canvas.set_right(right_tool_canvas);

        ui.canvas.set_center(shared_canvas);

        let mut status_canvas = TheCanvas::new();
        let mut statusbar = TheStatusbar::new(TheId::named("Statusbar"));
        statusbar.set_text(fl!("info_welcome"));
        status_canvas.set_widget(statusbar);

        ui.canvas.set_bottom(status_canvas);

        self.apply_toolbar_visibility(ui, ctx);
        self.apply_theme_preset(ui, ctx);
        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
        self.server_ctx.show_editing_geometry = self.option_show_gizmos;

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
                    } else if item_id.name == "MenuView::ToggleLeft" {
                        self.show_left_toolbar = !self.show_left_toolbar;
                        self.apply_toolbar_visibility(ui, ctx);
                        ui.set_widget_value(
                            "OptionLeftToolbarCB",
                            ctx,
                            TheValue::Bool(self.show_left_toolbar),
                        );
                    } else if item_id.name == "MenuView::ToggleRight" {
                        self.show_right_toolbar = !self.show_right_toolbar;
                        self.apply_toolbar_visibility(ui, ctx);
                        ui.set_widget_value(
                            "OptionRightToolbarCB",
                            ctx,
                            TheValue::Bool(self.show_right_toolbar),
                        );
                    } else if item_id.name == "MenuTheme::Dark" {
                        self.theme_preset = "Dark".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(0));
                    } else if item_id.name == "MenuTheme::Light" {
                        self.theme_preset = "Light".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(1));
                    } else if item_id.name == "MenuTheme::Slate" {
                        self.theme_preset = "Slate".to_string();
                        self.apply_theme_preset(ui, ctx);
                        ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(2));
                    } else if item_id.name == "MenuOption::Snap" {
                        self.option_snap_to_grid = !self.option_snap_to_grid;
                        self.server_ctx.snap_to_grid = self.option_snap_to_grid;
                        ui.set_widget_value(
                            "OptionSnapCB",
                            ctx,
                            TheValue::Bool(self.option_snap_to_grid),
                        );
                    } else if item_id.name == "MenuOption::Grid" {
                        self.option_show_grid = !self.option_show_grid;
                        ui.set_widget_value(
                            "OptionGridCB",
                            ctx,
                            TheValue::Bool(self.option_show_grid),
                        );
                    } else if item_id.name == "MenuOption::Gizmos" {
                        self.option_show_gizmos = !self.option_show_gizmos;
                        self.server_ctx.show_editing_geometry = self.option_show_gizmos;
                        ui.set_widget_value(
                            "OptionGizmoCB",
                            ctx,
                            TheValue::Bool(self.option_show_gizmos),
                        );
                    } else if item_id.name == "MenuHelp::Docs" {
                        _ = open::that("https://www.eldiron.com/docs");
                    } else if item_id.name == "MenuHelp::Examples" {
                        _ = open::that("https://www.eldiron.com/docs/games");
                    } else if item_id.name == "MenuHelp::About" {
                        ctx.ui.send(TheEvent::SetStatusText(
                            TheId::empty(),
                            "Encheament Engine: integrated 2D/3D editor shell.".to_string(),
                        ));
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
                        if let TheValue::Text(url) = value {
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
                        if let Some(tool_name) = id
                            .name
                            .strip_prefix("TopTool::")
                            .or_else(|| id.name.strip_prefix("RightTool::"))
                        {
                            ctx.ui.send(TheEvent::Custom(
                                TheId::named("Set Tool"),
                                TheValue::Text(tool_name.to_string()),
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
                        } else if id.name == "QuickThemeDark" {
                            self.theme_preset = "Dark".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(0));
                            redraw = true;
                        } else if id.name == "QuickThemeLight" {
                            self.theme_preset = "Light".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(1));
                            redraw = true;
                        } else if id.name == "QuickThemeSlate" {
                            self.theme_preset = "Slate".to_string();
                            self.apply_theme_preset(ui, ctx);
                            ui.set_widget_value("ThemePresetDropdown", ctx, TheValue::Int(2));
                            redraw = true;
                        } else if id.name == "QuickOptSnap" {
                            self.option_snap_to_grid = !self.option_snap_to_grid;
                            self.server_ctx.snap_to_grid = self.option_snap_to_grid;
                            ui.set_widget_value(
                                "OptionSnapCB",
                                ctx,
                                TheValue::Bool(self.option_snap_to_grid),
                            );
                            redraw = true;
                        } else if id.name == "QuickOptGrid" {
                            self.option_show_grid = !self.option_show_grid;
                            ui.set_widget_value(
                                "OptionGridCB",
                                ctx,
                                TheValue::Bool(self.option_show_grid),
                            );
                            redraw = true;
                        } else if id.name == "QuickOptGizmos" {
                            self.option_show_gizmos = !self.option_show_gizmos;
                            self.server_ctx.show_editing_geometry = self.option_show_gizmos;
                            ui.set_widget_value(
                                "OptionGizmoCB",
                                ctx,
                                TheValue::Bool(self.option_show_gizmos),
                            );
                            redraw = true;
                        }
                    }

                    if id.name == "Help" {
                        self.server_ctx.help_mode = state == TheWidgetState::Clicked;
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
                        crate::features::blueprint_system::launch_blueprint_editor();
                        _ = open::that("https://eldiron.com");
                        ctx.ui
                            .set_widget_state("Logo".to_string(), TheWidgetState::None);
                        ctx.ui.clear_hover();
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
                        }
                    } else if id.name == "OptionGridCB" {
                        if let TheValue::Bool(v) = value {
                            self.option_show_grid = v;
                        }
                    } else if id.name == "OptionGizmoCB" {
                        if let TheValue::Bool(v) = value {
                            self.option_show_gizmos = v;
                            self.server_ctx.show_editing_geometry = v;
                        }
                    } else if id.name == "OptionLeftToolbarCB" {
                        if let TheValue::Bool(v) = value {
                            self.show_left_toolbar = v;
                            self.apply_toolbar_visibility(ui, ctx);
                        }
                    } else if id.name == "OptionRightToolbarCB" {
                        if let TheValue::Bool(v) = value {
                            self.show_right_toolbar = v;
                            self.apply_toolbar_visibility(ui, ctx);
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
                    }
                }
                _ => {}
            }
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
