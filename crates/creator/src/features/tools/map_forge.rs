use egui::{self, Color32, RichText, Sense, Stroke};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapForgeTool {
    PaintTile,
    EraseTile,
    FillArea,
    PlaceObject,
    Select,
}

#[derive(Clone, Debug)]
pub struct MapForgeState {
    pub visible: bool,
    pub show_grid: bool,
    pub zoom: f32,
    pub offset: egui::Vec2,
    pub active_tool: MapForgeTool,
}

impl Default for MapForgeState {
    fn default() -> Self {
        Self {
            visible: false,
            show_grid: true,
            zoom: 1.0,
            offset: egui::Vec2::ZERO,
            active_tool: MapForgeTool::PaintTile,
        }
    }
}

impl MapForgeState {
    pub fn open(&mut self) {
        self.visible = true;
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.offset = egui::Vec2::ZERO;
    }

    pub fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                self.zoom = (self.zoom - 0.1).max(0.2);
            }
            ui.label(format!("Zoom: {:.1}x", self.zoom));
            if ui.button("+").clicked() {
                self.zoom = (self.zoom + 0.1).min(4.0);
            }
            if ui
                .button(if self.show_grid {
                    "Hide Grid"
                } else {
                    "Show Grid"
                })
                .clicked()
            {
                self.show_grid = !self.show_grid;
            }
            if ui.button("Reset Pan").clicked() {
                self.offset = egui::Vec2::ZERO;
            }
        });

        ui.separator();
        ui.horizontal_wrapped(|ui| {
            for (label, tool) in [
                ("Paint", MapForgeTool::PaintTile),
                ("Erase", MapForgeTool::EraseTile),
                ("Fill", MapForgeTool::FillArea),
                ("Object", MapForgeTool::PlaceObject),
                ("Select", MapForgeTool::Select),
            ] {
                let selected = self.active_tool == tool;
                if ui.selectable_label(selected, label).clicked() {
                    self.active_tool = tool;
                }
            }
        });
    }

    pub fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), ui.available_height() - 40.0),
            Sense::drag(),
        );

        if response.dragged() {
            self.offset += response.drag_delta();
        }

        ui.painter().rect_filled(rect, 0.0, Color32::BLACK);
        ui.painter()
            .rect_stroke(rect, 0.0, Stroke::new(2.0, Color32::WHITE));

        if self.show_grid {
            let grid_size = 32.0 * self.zoom;
            let start_x = rect.left() + self.offset.x % grid_size;
            let start_y = rect.top() + self.offset.y % grid_size;
            let color = Color32::from_gray(60);

            let mut x = start_x;
            while x < rect.right() {
                ui.painter().line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    Stroke::new(1.0, color),
                );
                x += grid_size;
            }

            let mut y = start_y;
            while y < rect.bottom() {
                ui.painter().line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    Stroke::new(1.0, color),
                );
                y += grid_size;
            }
        }

        let tool_name = match self.active_tool {
            MapForgeTool::PaintTile => "Paint",
            MapForgeTool::EraseTile => "Erase",
            MapForgeTool::FillArea => "Fill",
            MapForgeTool::PlaceObject => "Object",
            MapForgeTool::Select => "Select",
        };

        ui.painter().text(
            rect.center_top() + egui::vec2(0.0, 20.0),
            egui::Align2::CENTER_CENTER,
            RichText::new(format!("MapForge ({tool_name})"))
                .color(Color32::WHITE)
                .size(24.0),
            egui::FontId::proportional(24.0),
            Color32::WHITE,
        );
    }
}
