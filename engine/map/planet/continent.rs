//! Continent: large landmass on a planet.

pub struct Continent {
    pub name: String,
    pub kingdoms: Vec<String>,
}

impl Continent {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string(), kingdoms: Vec::new() }
    }
}
