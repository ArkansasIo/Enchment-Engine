//! Dungeon: procedural dungeon layout and rooms.

pub struct Dungeon {
    pub rooms: Vec<Room>,
    pub grid: super::grid::Grid,
}

pub struct Room {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Dungeon {
    pub fn new(grid: super::grid::Grid) -> Self {
        Self { rooms: Vec::new(), grid }
    }
}
