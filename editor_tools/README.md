# Enchentment Engine Editor Tools

This folder contains all modular editor tools for the Enchentment Engine. Each tool is implemented as a Rust module with a UI stub and an egui-based implementation file. Extend these tools to add new features, editors, and workflows.

## Tool List

- **Asset Browser**: Browse, search, and inspect all assets by category.
- **Monster Editor**: Create/edit monsters: stats, loot, AI, animation, biome, etc.
- **Item Editor**: Create/edit items, weapons, armor, shields.
- **Loot Table Editor**: Visual editor for loot tables: entries, weights, rarity, min/max.
- **Building Editor**: Create/edit buildings, props, and placeable objects.
- **Procedural Rule Editor**: Design procedural generation rules for maps, dungeons, etc.
- **Tilemap Tool**: Paint tilemaps, place buildings/props, validate placement.
- **Animation Editor**: Assign frames, set FPS, preview animations.
- **Stat Curve Visualizer**: Visualize and edit stat/XP/level curves.
- **Save/Export Tool**: Manage save/export of projects, assets, and data.

## File Structure

- `asset_browser.rs` / `asset_browser_impl.rs`: Asset browser tool and UI logic
- `monster_editor.rs` / `monster_editor_impl.rs`: Monster editor tool and UI logic
- `item_editor.rs` / `item_editor_impl.rs`: Item editor tool and UI logic
- `loot_table_editor.rs` / `loot_table_editor_impl.rs`: Loot table editor tool and UI logic
- `building_editor.rs` / `building_editor_impl.rs`: Building editor tool and UI logic
- `procedural_rule_editor.rs` / `procedural_rule_editor_impl.rs`: Procedural rule editor tool and UI logic
- `tilemap_tool.rs` / `tilemap_tool_impl.rs`: Tilemap tool and UI logic
- `animation_editor.rs` / `animation_editor_impl.rs`: Animation editor tool and UI logic
- `stat_curve_visualizer.rs` / `stat_curve_visualizer_impl.rs`: Stat curve visualizer tool and UI logic
- `save_export.rs` / `save_export_impl.rs`: Save/export tool and UI logic
- `main_editor.rs`: Main editor UI layout and tool switching
- `mod.rs`: Module index for all tools
- `ui_wireframe.md`: UI wireframe and design notes

---
Extend or add new tools by following the modular pattern in this folder. Each tool should have a stub and an egui-based implementation for rapid development.
