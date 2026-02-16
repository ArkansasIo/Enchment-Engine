//! Special: custom or rare map detail.

pub struct SpecialDetail {
    pub kind: String,
    pub coords: (i32, i32),
    pub data: String,
}

impl SpecialDetail {
    pub fn new(kind: &str, coords: (i32, i32), data: &str) -> Self {
        Self { kind: kind.to_string(), coords, data: data.to_string() }
    }
}
