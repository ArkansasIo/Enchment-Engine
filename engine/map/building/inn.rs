//! Inn building definition.

pub struct Inn {
    pub name: String,
    pub beds: u32,
}

impl Inn {
    pub fn new(name: &str, beds: u32) -> Self {
        Self { name: name.to_string(), beds }
    }
}
