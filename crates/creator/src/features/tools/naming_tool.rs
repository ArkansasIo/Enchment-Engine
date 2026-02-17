//! Naming tools for places, zones, and subzones (cities, towns, kingdoms, etc.)

use super::gui::SimpleEntry;

pub struct NamingTool;

impl NamingTool {
    pub fn add_entry(list: &mut Vec<SimpleEntry>, name: String) {
        list.push(SimpleEntry { name });
    }

    pub fn edit_entry(list: &mut Vec<SimpleEntry>, idx: usize, name: String) {
        if let Some(entry) = list.get_mut(idx) {
            entry.name = name;
        }
    }

    pub fn remove_entry(list: &mut Vec<SimpleEntry>, idx: usize) {
        if idx < list.len() {
            list.remove(idx);
        }
    }

    pub fn auto_generate_name(prefix: &str, idx: usize) -> String {
        format!("{} {}", prefix, idx + 1)
    }
}
