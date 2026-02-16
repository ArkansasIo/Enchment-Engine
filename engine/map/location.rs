//! Location: named places (towns, dungeons, POIs).

pub struct Location {
    pub name: String,
    pub coords: (i32, i32),
    pub kind: LocationKind,
}

pub enum LocationKind {
    Town,
    Dungeon,
    Landmark,
    Special(String),
}
