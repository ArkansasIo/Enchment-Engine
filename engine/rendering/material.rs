//! Material system: node-based and PBR materials.

pub struct Material {
    pub name: String,
}

impl Material {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn apply(&self) {
        // TODO: implement material application
    }
}
