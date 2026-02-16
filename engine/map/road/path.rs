//! Path: minor road or trail.

pub struct Path {
    pub name: String,
    pub length: f32,
    pub from: (i32, i32),
    pub to: (i32, i32),
}

impl Path {
    pub fn new(name: &str, length: f32, from: (i32, i32), to: (i32, i32)) -> Self {
        Self { name: name.to_string(), length, from, to }
    }
}
