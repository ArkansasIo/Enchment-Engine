//! Bridge: structure crossing a river or gap.

pub struct Bridge {
    pub name: String,
    pub location: (i32, i32),
    pub length: f32,
}

impl Bridge {
    pub fn new(name: &str, location: (i32, i32), length: f32) -> Self {
        Self { name: name.to_string(), location, length }
    }
}
