//! Unreal 5-style Blueprint Editor system for visual scripting

use egui::*;

/// Represents a node in the blueprint editor.
#[derive(Clone)]
pub struct BlueprintNode {
    pub id: usize,
    pub pos: Pos2,
    pub title: String,
}

/// Represents a connection between two nodes.
#[derive(Clone)]
pub struct BlueprintConnection {
    pub from: usize,
    pub to: usize,
}

pub struct BlueprintEditorGui {
    pub show: bool,
    pub nodes: Vec<BlueprintNode>,
    pub connections: Vec<BlueprintConnection>,
    pub dragging_node: Option<usize>,
    pub drag_offset: Pos2,
    pub next_node_id: usize,
}


impl BlueprintEditorGui {
    pub fn new() -> Self {
        Self {
            show: false,
            nodes: vec![
                BlueprintNode { id: 0, pos: Pos2::new(100.0, 100.0), title: "Start".to_string() },
                BlueprintNode { id: 1, pos: Pos2::new(300.0, 200.0), title: "Action".to_string() },
            ],
            connections: vec![BlueprintConnection { from: 0, to: 1 }],
            dragging_node: None,
            drag_offset: Pos2::ZERO,
            next_node_id: 2,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("Blueprint Editor").open(&mut self.show).show(ctx, |ui| {
            ui.heading("Blueprint Node Graph (Unreal-style)");
            ui.label("Drag nodes, connect them, and right-click to add new nodes.");

            let graph_rect = ui.available_rect_before_wrap();
            let painter = ui.painter_at(graph_rect);

            // Draw connections
            for conn in &self.connections {
                let from = self.nodes.iter().find(|n| n.id == conn.from);
                let to = self.nodes.iter().find(|n| n.id == conn.to);
                if let (Some(from), Some(to)) = (from, to) {
                    let start = from.pos + Pos2::new(80.0, 20.0);
                    let end = to.pos + Pos2::new(0.0, 20.0);
                    painter.line_segment([start, end], (2.0, Color32::LIGHT_BLUE));
                }
            }

            // Node drag-and-drop logic
            let mut drag_released = false;
            let mut drag_start = None;
            let mut drag_end = None;
            let mut new_connection: Option<(usize, usize)> = None;

            for node in &mut self.nodes {
                let node_rect = Rect::from_min_size(graph_rect.min + node.pos, vec2(120.0, 40.0));
                let response = ui.allocate_rect(node_rect, Sense::click_and_drag());

                // Draw node
                painter.rect_filled(node_rect, 6.0, Color32::from_rgb(40, 60, 120));
                painter.text(node_rect.center(), Align2::CENTER_CENTER, &node.title, TextStyle::Button.resolve(ui.style()), Color32::WHITE);

                // Drag logic
                if response.drag_started() {
                    self.dragging_node = Some(node.id);
                    self.drag_offset = response.interact_pointer_pos().unwrap_or(Pos2::ZERO) - (graph_rect.min + node.pos);
                }
                if response.dragged() && self.dragging_node == Some(node.id) {
                    if let Some(pointer) = response.interact_pointer_pos() {
                        node.pos = pointer - graph_rect.min - self.drag_offset;
                    }
                }
                if response.drag_released() && self.dragging_node == Some(node.id) {
                    self.dragging_node = None;
                    drag_released = true;
                }

                // Connection drag (from right edge)
                let port_rect = Rect::from_min_size(node_rect.right_center() + vec2(-10.0, -10.0), vec2(20.0, 20.0));
                let port_resp = ui.allocate_rect(port_rect, Sense::click());
                painter.circle_filled(port_rect.center(), 6.0, Color32::YELLOW);
                if port_resp.drag_started() {
                    drag_start = Some(node.id);
                }
                if port_resp.dragged() && drag_start.is_some() {
                    // Draw preview line
                    if let Some(pointer) = port_resp.interact_pointer_pos() {
                        painter.line_segment([port_rect.center(), pointer], (2.0, Color32::YELLOW));
                    }
                }
                if port_resp.drag_released() && drag_start.is_some() {
                    drag_end = Some(node.id);
                }
            }

            // Add connection if drag finished between two nodes
            if let (Some(from), Some(to)) = (drag_start, drag_end) {
                if from != to && !self.connections.iter().any(|c| c.from == from && c.to == to) {
                    self.connections.push(BlueprintConnection { from, to });
                }
            }

            // Right-click to add a new node
            if ui.ctx().input(|i| i.pointer.secondary_down()) && ui.rect_contains_pointer(graph_rect) {
                if let Some(pointer) = ui.ctx().pointer_latest_pos() {
                    if ui.ctx().input(|i| i.pointer.any_released()) {
                        self.nodes.push(BlueprintNode {
                            id: self.next_node_id,
                            pos: pointer - graph_rect.min,
                            title: format!("Node {}", self.next_node_id),
                        });
                        self.next_node_id += 1;
                    }
                }
            }
        });
    }
}
