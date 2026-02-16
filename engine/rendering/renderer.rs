//! Renderer: main entry for all graphics output.

pub struct Renderer {
    pub width: u32,
    pub height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn render(&self) {
        // TODO: implement rendering logic
    }
}
