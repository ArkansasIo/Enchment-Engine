use egui::*;
use crate::features::tools::gui::{ItemEntry, ItemType};

pub struct ItemsMenuState {
    pub item_name: String,
    pub item_desc: String,
    pub item_value: i32,
    pub item_edit_idx: Option<usize>,
}

impl ItemsMenuState {
    pub fn new() -> Self {
        Self {
            item_name: String::new(),
            item_desc: String::new(),
            item_value: 0,
            item_edit_idx: None,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, all_items: &mut Vec<ItemEntry>) {
        egui::Window::new("Items").default_pos(egui::pos2(ctx.screen_rect().right() - 350.0, 60.0)).show(ctx, |ui| {
            ui.heading("Item List");
            let mut remove_idx = None;
            for (i, item) in all_items.iter().enumerate() {
                ui.horizontal(|h| {
                    h.label(&item.name);
                    if h.button("Edit").clicked() {
                        self.item_edit_idx = Some(i);
                        self.item_name = item.name.clone();
                        self.item_desc = item.description.clone();
                        self.item_value = item.value;
                    }
                    if h.button("Remove").clicked() {
                        remove_idx = Some(i);
                    }
                });
            }
            if let Some(idx) = remove_idx {
                all_items.remove(idx);
            }
            ui.separator();
            if let Some(edit_idx) = self.item_edit_idx {
                ui.label("Edit Item:");
                ui.text_edit_singleline(&mut self.item_name);
                ui.text_edit_singleline(&mut self.item_desc);
                ui.add(egui::DragValue::new(&mut self.item_value));
                if ui.button("Save").clicked() {
                    if !self.item_name.trim().is_empty() {
                        all_items[edit_idx].name = self.item_name.clone();
                        all_items[edit_idx].description = self.item_desc.clone();
                        all_items[edit_idx].value = self.item_value;
                        self.item_edit_idx = None;
                        self.item_name.clear();
                        self.item_desc.clear();
                        self.item_value = 0;
                    }
                }
                if ui.button("Cancel").clicked() {
                    self.item_edit_idx = None;
                    self.item_name.clear();
                    self.item_desc.clear();
                    self.item_value = 0;
                }
            } else {
                ui.label("Add New Item:");
                ui.text_edit_singleline(&mut self.item_name);
                ui.text_edit_singleline(&mut self.item_desc);
                ui.add(egui::DragValue::new(&mut self.item_value));
                if ui.button("Add").clicked() {
                    if !self.item_name.trim().is_empty() {
                        all_items.push(ItemEntry {
                            name: self.item_name.clone(),
                            item_type: ItemType::Misc,
                            subtype: String::new(),
                            description: self.item_desc.clone(),
                            stats: vec![],
                            rarity: "Common".into(),
                            value: self.item_value,
                            usable_in_battle: false,
                            usable_in_field: false,
                        });
                        self.item_name.clear();
                        self.item_desc.clear();
                        self.item_value = 0;
                    }
                }
            }
        });
    }
}
