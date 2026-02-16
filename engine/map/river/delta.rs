//! Delta: river delta at the mouth.

pub struct Delta {
    pub branches: u32,
    pub mouth_coords: (i32, i32),
}

impl Delta {
    pub fn new(branches: u32, mouth_coords: (i32, i32)) -> Self {
        Self { branches, mouth_coords }
    }
}
