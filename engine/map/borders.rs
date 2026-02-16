//! Borders: political or natural boundaries on the map.

pub struct Border {
    pub name: String,
    pub kind: BorderKind,
    pub points: Vec<(i32, i32)>,
}

pub enum BorderKind {
    Political,
    Natural,
    Custom(String),
}

impl Border {
    pub fn new(name: &str, kind: BorderKind, points: Vec<(i32, i32)>) -> Self {
        Self { name: name.to_string(), kind, points }
    }

    pub fn generate_straight_border(name: &str, kind: BorderKind, from: (i32, i32), to: (i32, i32)) -> Self {
        let mut points = Vec::new();
        let (mut x, mut y) = from;
        let (tx, ty) = to;
        while (x, y) != (tx, ty) {
            if x < tx { x += 1; } else if x > tx { x -= 1; }
            if y < ty { y += 1; } else if y > ty { y -= 1; }
            points.push((x, y));
        }
        Self { name: name.to_string(), kind, points }
    }
}
