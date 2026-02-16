//! Demo: usage example for map generation and navigation.

use super::generator::MapGenerator;

pub fn run_map_demo() {
    let map = MapGenerator::generate_basic_map(16, 16);
    println!("Generated map with {} zones.", map.zones.len());
    let dungeon = MapGenerator::generate_dungeon(16, 16);
    println!("Generated dungeon with {} rooms.", dungeon.rooms.len());
}
