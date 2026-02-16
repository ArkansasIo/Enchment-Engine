//! Label: textual annotation for map features.

pub struct Label {
    pub text: String,
    pub coords: (i32, i32),
}

impl Label {
    pub fn new(text: &str, coords: (i32, i32)) -> Self {
        Self { text: text.to_string(), coords }
    }
}
