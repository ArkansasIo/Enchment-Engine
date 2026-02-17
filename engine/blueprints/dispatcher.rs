//! Blueprint Dispatcher: event broadcaster

#[derive(Debug, Clone)]
pub struct Dispatcher {
    pub name: String,
}

impl Dispatcher {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
}
