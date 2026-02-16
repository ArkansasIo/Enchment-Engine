//! MapGenerator: procedural map and dungeon generation logic.

use super::{MapLayout, Zone, Grid, Dungeon};

pub struct MapGenerator;

impl MapGenerator {
    pub fn generate_basic_map(width: u32, height: u32) -> MapLayout {
        let mut layout = MapLayout::new(width, height);
        // Example: create a single zone with a grid
        let grid = Grid::new(width, height);
        let zone = Zone::new("Central Zone", grid);
        layout.zones.push(zone);
        layout
    }

    pub fn generate_dungeon(width: u32, height: u32) -> Dungeon {
        let grid = Grid::new(width, height);
        let mut dungeon = Dungeon::new(grid);
        // Example: add a single room
        dungeon.rooms.push(super::Room { x: 2, y: 2, width: 5, height: 5 });
        dungeon
    }
}
