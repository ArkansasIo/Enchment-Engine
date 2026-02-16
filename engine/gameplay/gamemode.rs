//! GameMode: defines game rules and flow.

pub struct GameMode {
    pub name: String,
}

impl GameMode {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn start(&self) {
        // TODO: implement game start logic
    }
}
