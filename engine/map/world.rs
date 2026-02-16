//! World: collection of maps and global context.

pub struct World {
    pub maps: Vec<MapLayout>,
}

use super::layout::MapLayout;

impl World {
    pub fn new() -> Self {
        Self { maps: Vec::new() }
    }
}
