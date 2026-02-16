//! Tributary: secondary river joining a main river.

pub struct Tributary {
    pub name: String,
    pub length: f32,
    pub joins_at: (i32, i32),
}

impl Tributary {
    pub fn new(name: &str, length: f32, joins_at: (i32, i32)) -> Self {
        Self { name: name.to_string(), length, joins_at }
    }
}
