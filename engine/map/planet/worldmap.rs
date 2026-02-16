//! WorldMap: entire planet map system.

pub struct WorldMap {
    pub name: String,
    pub continents: Vec<Continent>,
    pub oceans: Vec<Ocean>,
}

use super::continent::Continent;
use super::ocean::Ocean;

impl WorldMap {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), continents: Vec::new(), oceans: Vec::new() }
    }
}
