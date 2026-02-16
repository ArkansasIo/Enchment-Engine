//! ImportExport: support for importing/exporting map data (e.g., watabou.github.io JSON).

use super::{MapLayout, Dungeon};

pub struct ImportExport;

impl ImportExport {
    pub fn import_map_from_json(_json: &str) -> Option<MapLayout> {
        // TODO: Parse JSON and construct MapLayout
        None
    }
    pub fn export_map_to_json(_map: &MapLayout) -> String {
        // TODO: Serialize MapLayout to JSON
        "{}".to_string()
    }
    pub fn import_dungeon_from_json(_json: &str) -> Option<Dungeon> {
        // TODO: Parse JSON and construct Dungeon
        None
    }
    pub fn export_dungeon_to_json(_dungeon: &Dungeon) -> String {
        // TODO: Serialize Dungeon to JSON
        "{}".to_string()
    }
}
