//! Realm kingdom definition.

pub struct Realm {
    pub name: String,
    pub ruler: String,
    pub population: u64,
}

impl Realm {
    pub fn new(name: &str, ruler: &str, population: u64) -> Self {
        Self { name: name.to_string(), ruler: ruler.to_string(), population }
    }
}
