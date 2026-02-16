//! Gameplay Ability System (GAS).

pub struct Ability {
    pub name: String,
}

impl Ability {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn activate(&self) {
        // TODO: implement ability activation
    }
}
