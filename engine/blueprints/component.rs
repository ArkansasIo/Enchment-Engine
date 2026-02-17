//! Blueprint Component: reusable part of an Actor Blueprint

#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub component_type: String,
}

impl Component {
    pub fn new(name: &str, component_type: &str) -> Self {
        Self { name: name.to_string(), component_type: component_type.to_string() }
    }
}
