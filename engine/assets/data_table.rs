//! DataTable: structured data asset system.

pub struct DataTable {
    pub name: String,
}

impl DataTable {
    pub fn new(name: &str) -> Self {
        Self { name: name.to_string() }
    }
    pub fn load(&self) {
        // TODO: implement data loading
    }
}
