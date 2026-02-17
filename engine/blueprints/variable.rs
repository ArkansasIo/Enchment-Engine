//! Blueprint Variable: state/data in a Blueprint

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: String,
}

impl Variable {
    pub fn new(name: &str, var_type: &str) -> Self {
        Self { name: name.to_string(), var_type: var_type.to_string() }
    }
}
