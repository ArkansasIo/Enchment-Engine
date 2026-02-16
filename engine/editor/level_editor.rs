//! Level Editor: main editor viewport and logic.

pub struct LevelEditor {
    pub active_level: String,
}

impl LevelEditor {
    pub fn new(level: &str) -> Self {
        Self { active_level: level.to_string() }
    }
    pub fn open(&mut self, level: &str) {
        self.active_level = level.to_string();
    }
    pub fn save(&self) {
        // TODO: implement save logic
    }
}
