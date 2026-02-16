use serde::{Serialize, Deserialize};
use egui::{Color32, FontFamily, FontId, Ui, ComboBox, CentralPanel, SidePanel, TopBottomPanel, Context};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct Theme {
    pub name: String,
    pub background: Color32,
    pub foreground: Color32,
    pub accent: Color32,
    pub font: FontId,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".into(),
            background: Color32::from_rgb(240, 240, 240),
            foreground: Color32::BLACK,
            accent: Color32::from_rgb(0, 120, 220),
            font: FontId::proportional(16.0),
        }
    }
    pub fn dark() -> Self {
        Self {
            name: "Dark".into(),
            background: Color32::from_rgb(30, 30, 30),
            foreground: Color32::WHITE,
            accent: Color32::from_rgb(0, 180, 255),
            font: FontId::proportional(16.0),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum DockPosition {
    Left,
    Right,
    Top,
    Bottom,
    Floating,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct PanelLayout {
    pub panel_id: String,
    pub position: DockPosition,
    pub size: (u32, u32),
    pub visible: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkspaceLayout {
    pub panels: Vec<PanelLayout>,
    pub theme: String,
}

impl WorkspaceLayout {
    pub fn default() -> Self {
        Self {
            panels: vec![
                PanelLayout { panel_id: "ToolbarLeft".into(), position: DockPosition::Left, size: (60, 600), visible: true },
                PanelLayout { panel_id: "ToolbarTop".into(), position: DockPosition::Top, size: (800, 40), visible: true },
            ],
            theme: "Light".into(),
        }
    }
}

pub struct ThemeSettings {
    pub available_themes: Vec<Theme>,
    pub current_theme: String,
    pub workspace_layout: WorkspaceLayout,
}

impl ThemeSettings {
    pub fn new() -> Self {
        Self {
            available_themes: vec![Theme::light(), Theme::dark()],
            current_theme: "Light".into(),
            workspace_layout: WorkspaceLayout::default(),
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        self.current_theme = theme_name.into();
    }

    pub fn current_theme(&self) -> &Theme {
        self.available_themes.iter().find(|t| t.name == self.current_theme).unwrap_or(&self.available_themes[0])
    }

    pub fn show_theme_settings(&mut self, ui: &mut Ui) {
        ui.heading("Theme");
        ComboBox::from_label("Select Theme")
            .selected_text(&self.current_theme)
            .show_ui(ui, |ui| {
                for theme in &self.available_themes {
                    ui.selectable_value(&mut self.current_theme, theme.name.clone(), &theme.name);
                }
            });
    }

    pub fn show_layout_settings(&mut self, ui: &mut Ui) {
        ui.heading("Layout");
        for panel in &mut self.workspace_layout.panels {
            ui.horizontal(|ui| {
                ui.label(&panel.panel_id);
                ComboBox::from_id_source(&panel.panel_id)
                    .selected_text(format!("{:?}", panel.position))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut panel.position, DockPosition::Left, "Left");
                        ui.selectable_value(&mut panel.position, DockPosition::Right, "Right");
                        ui.selectable_value(&mut panel.position, DockPosition::Top, "Top");
                        ui.selectable_value(&mut panel.position, DockPosition::Bottom, "Bottom");
                        ui.selectable_value(&mut panel.position, DockPosition::Floating, "Floating");
                    });
                ui.checkbox(&mut panel.visible, "Visible");
            });
        }
    }

    pub fn apply_theme(&self, ctx: &Context) {
        let theme = self.current_theme();
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = theme.background;
        style.visuals.widgets.active.bg_fill = theme.accent;
        style.visuals.widgets.inactive.bg_fill = theme.background;
        style.visuals.widgets.hovered.bg_fill = theme.accent;
        style.visuals.override_text_color = Some(theme.foreground);
        ctx.set_style(style);
    }
}

// Example egui integration in your main UI code:
//
// fn ui(&mut self, ctx: &egui::Context) {
//     self.theme_settings.apply_theme(ctx);
//     egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
//         self.theme_settings.show_theme_settings(ui);
//     });
//     egui::SidePanel::left("left_toolbar").show(ctx, |ui| {
//         // ...
//     });
//     // ...
//     egui::CentralPanel::default().show(ctx, |ui| {
//         // ...
//     });
// }
