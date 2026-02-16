//! Highway: major road connecting cities.

pub struct Highway {
    pub name: String,
    pub length: f32,
    pub from: (i32, i32),
    pub to: (i32, i32),
    pub path: Vec<(i32, i32)>,
}

impl Highway {
    pub fn new(name: &str, from: (i32, i32), to: (i32, i32)) -> Self {
        let path = Highway::generate_path(from, to);
        let length = path.len() as f32;
        Self { name: name.to_string(), length, from, to, path }
    }

    pub fn generate_path(from: (i32, i32), to: (i32, i32)) -> Vec<(i32, i32)> {
        // Simple straight-line road for demo; replace with A* for realism
        let mut path = Vec::new();
        let (mut x, mut y) = from;
        let (tx, ty) = to;
        while (x, y) != (tx, ty) {
            if x < tx { x += 1; } else if x > tx { x -= 1; }
            if y < ty { y += 1; } else if y > ty { y -= 1; }
            path.push((x, y));
        }
        path
    }
}
