//! Note: user or system annotation for map features.

pub struct Note {
    pub text: String,
    pub coords: (i32, i32),
}

impl Note {
    pub fn new(text: &str, coords: (i32, i32)) -> Self {
        Self { text: text.to_string(), coords }
    }
}
