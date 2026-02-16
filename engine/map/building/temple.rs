//! Temple building definition.

pub struct Temple {
    pub deity: String,
    pub priests: u32,
}

impl Temple {
    pub fn new(deity: &str, priests: u32) -> Self {
        Self { deity: deity.to_string(), priests }
    }
}
