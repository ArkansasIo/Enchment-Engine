//! MetaSound: procedural audio system.

pub struct MetaSound {
    pub name: String,
}

impl MetaSound {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn play(&self) {
        // TODO: implement audio playback
    }
}
