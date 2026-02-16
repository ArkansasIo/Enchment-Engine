//! Details: extra map features (labels, icons, etc).

pub struct MapDetail {
    pub name: String,
    pub coords: (i32, i32),
    pub kind: DetailKind,
}

pub enum DetailKind {
    Label,
    Icon(String),
    Special(String),
}

pub mod details;
