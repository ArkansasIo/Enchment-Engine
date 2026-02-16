//! Principality kingdom definition.

pub struct Principality {
    pub name: String,
    pub prince: String,
}

impl Principality {
    pub fn new(name: &str, prince: &str) -> Self {
        Self { name: name.to_string(), prince: prince.to_string() }
    }
}
