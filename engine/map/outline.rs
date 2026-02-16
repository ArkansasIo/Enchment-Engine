//! Outline: map outlines, borders, and region shapes.

pub struct Outline {
    pub points: Vec<(i32, i32)>,
}

impl Outline {
    pub fn new(points: Vec<(i32, i32)>) -> Self {
        Self { points }
    }
}
