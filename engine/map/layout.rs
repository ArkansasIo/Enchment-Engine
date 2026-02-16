//! MapLayout: high-level map structure (inspired by watabou.github.io)

pub struct MapLayout {
    pub width: u32,
    pub height: u32,
    pub zones: Vec<Zone>,
}

use super::zone::Zone;

impl MapLayout {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, zones: Vec::new() }
    }
}
