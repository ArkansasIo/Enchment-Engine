//! Empire kingdom definition.

pub struct Empire {
    pub name: String,
    pub emperor: String,
    pub vassals: u32,
}

impl Empire {
    pub fn new(name: &str, emperor: &str, vassals: u32) -> Self {
        Self { name: name.to_string(), emperor: emperor.to_string(), vassals }
    }
}
