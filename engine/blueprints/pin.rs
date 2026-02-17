//! Blueprint Pin: input/output for nodes

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PinKind {
    Exec,
    Data(String), // type name
}

#[derive(Debug, Clone)]
pub struct Pin {
    pub name: String,
    pub kind: PinKind,
    pub is_input: bool,
}

impl Pin {
    pub fn new(name: &str, kind: PinKind, is_input: bool) -> Self {
        Self { name: name.to_string(), kind, is_input }
    }
}
