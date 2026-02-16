//! Grid: 2D grid for map layout, tiles, and navigation.

pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub cells: Vec<Cell>,
}

pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub terrain: TerrainType,
}

pub enum TerrainType {
    Floor,
    Wall,
    Water,
    Grass,
    Road,
    Special(String),
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let mut cells = Vec::new();
        for y in 0..height {
            for x in 0..width {
                cells.push(Cell { x, y, terrain: TerrainType::Floor });
            }
        }
        Self { width, height, cells }
    }
}
