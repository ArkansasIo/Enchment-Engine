//! MapLogic: core logic for procedural map generation and manipulation.

use super::{MapLayout, Zone, Grid, Dungeon, Location};

pub struct MapLogic;

impl MapLogic {
    pub fn generate_city_map(width: u32, height: u32) -> MapLayout {
        // TODO: Generate a city map layout (inspired by watabou.github.io)
        MapLayout::new(width, height)
    }
    pub fn generate_dungeon_map(width: u32, height: u32) -> Dungeon {
        // TODO: Generate a dungeon map layout (inspired by watabou.github.io)
        Dungeon::new(Grid::new(width, height))
    }
    pub fn place_locations(_map: &mut MapLayout) {
        // TODO: Place towns, dungeons, and POIs
    }
}
