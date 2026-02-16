//! POI: point of interest on the map.

pub struct POI {
    pub name: String,
    pub coords: (i32, i32),
    pub description: String,
}

impl POI {
    pub fn new(name: &str, coords: (i32, i32), description: &str) -> Self {
        Self { name: name.to_string(), coords, description: description.to_string() }
    }
}
