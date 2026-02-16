//! Icon: graphical symbol for map features.

pub struct Icon {
    pub kind: String,
    pub coords: (i32, i32),
}

impl Icon {
    pub fn new(kind: &str, coords: (i32, i32)) -> Self {
        Self { kind: kind.to_string(), coords }
    }
}
