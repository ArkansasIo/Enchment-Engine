//! MainRiver: primary river system.

pub struct MainRiver {
    pub name: String,
    pub length: f32,
    pub source: (i32, i32),
    pub mouth: (i32, i32),
    pub path: Vec<(i32, i32)>,
}

impl MainRiver {
    pub fn new(name: &str, source: (i32, i32), mouth: (i32, i32)) -> Self {
        let path = MainRiver::generate_path(source, mouth);
        let length = path.len() as f32;
        Self { name: name.to_string(), length, source, mouth, path }
    }

    pub fn generate_path(source: (i32, i32), mouth: (i32, i32)) -> Vec<(i32, i32)> {
        // Simple straight-line river for demo; replace with noise/pathfinding for realism
        let mut path = Vec::new();
        let (mut x, mut y) = source;
        let (mx, my) = mouth;
        while (x, y) != (mx, my) {
            if x < mx { x += 1; } else if x > mx { x -= 1; }
            if y < my { y += 1; } else if y > my { y -= 1; }
            path.push((x, y));
        }
        path
    }
}
