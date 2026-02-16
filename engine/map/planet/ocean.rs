//! Ocean: large body of water on a planet.

pub struct Ocean {
    pub name: String,
    pub area: f64,
}

impl Ocean {
    pub fn new(name: &str, area: f64) -> Self {
        Self { name: name.to_string(), area }
    }
}
