//! CustomOverlay: user-defined overlays for maps.

pub struct CustomOverlay {
    pub name: String,
    pub data: Vec<(i32, i32)>,
    pub description: String,
}

impl CustomOverlay {
    pub fn new(name: &str, data: Vec<(i32, i32)>, description: &str) -> Self {
        Self { name: name.to_string(), data, description: description.to_string() }
    }
}
