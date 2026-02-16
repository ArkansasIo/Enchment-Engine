// Tilemap Tool UI Implementation for egui
// Supports tile painting, placement, and validation

use egui::*;

const MAP_WIDTH: usize = 32;
const MAP_HEIGHT: usize = 32;
const TILESET: [&str; 4] = ["Grass", "Water", "Wall", "Sand"];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Grass,
    Water,
    Wall,
    Sand,
}

impl TileType {
    pub fn all() -> &'static [TileType] {
        &[TileType::Grass, TileType::Water, TileType::Wall, TileType::Sand]
    }
    pub fn name(&self) -> &'static str {
        match self {
            TileType::Grass => "Grass",
            TileType::Water => "Water",
            TileType::Wall => "Wall",
            TileType::Sand => "Sand",
        }
    }
}

pub struct TilemapToolImpl {
    pub map: [[TileType; MAP_WIDTH]; MAP_HEIGHT],
    pub selected_tile: TileType,
    pub status: String,
}

impl TilemapToolImpl {
    pub fn new() -> Self {
        Self {
            map: [[TileType::Grass; MAP_WIDTH]; MAP_HEIGHT],
            selected_tile: TileType::Grass,
            status: "Ready".to_string(),
        }
    }
    pub fn paint_tile(&mut self, x: usize, y: usize) {
        if x < MAP_WIDTH && y < MAP_HEIGHT {
            self.map[y][x] = self.selected_tile;
            self.status = format!("Painted {} at ({}, {})", self.selected_tile.name(), x, y);
        }
    }
    pub fn validate(&mut self) {
        // Example: No isolated water tiles
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                if self.map[y][x] == TileType::Water {
                    let mut surrounded = true;
                    for dy in [-1, 0, 1] {
                        for dx in [-1, 0, 1] {
                            if dx == 0 && dy == 0 { continue; }
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && nx < MAP_WIDTH as isize && ny >= 0 && ny < MAP_HEIGHT as isize {
                                if self.map[ny as usize][nx as usize] != TileType::Water {
                                    surrounded = false;
                                }
                            }
                        }
                    }
                    if !surrounded {
                        self.status = format!("Warning: Water at ({}, {}) is not surrounded!", x, y);
                        return;
                    }
                }
            }
        }
        self.status = "Validation passed".to_string();
    }
    pub fn ui(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("tile_palette").show(ctx, |ui| {
            ui.heading("Tile Palette");
            for &tile in TileType::all() {
                if ui.selectable_label(self.selected_tile == tile, tile.name()).clicked() {
                    self.selected_tile = tile;
                }
            }
            if ui.button("Validate Map").clicked() {
                self.validate();
            }
            ui.label(&self.status);
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Tilemap Editor");
            for y in 0..MAP_HEIGHT {
                ui.horizontal(|ui| {
                    for x in 0..MAP_WIDTH {
                        let label = match self.map[y][x] {
                            TileType::Grass => "🟩",
                            TileType::Water => "🟦",
                            TileType::Wall => "⬛",
                            TileType::Sand => "🟨",
                        };
                        let response = ui.button(label);
                        if response.clicked() {
                            self.paint_tile(x, y);
                        }
                    }
                });
            }
        });
    }
}
