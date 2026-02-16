# Enchentment Engine Editor UI Wireframe & Mockup

## Main Layout
- **Sidebar (left):**
  - Asset Browser (icon: 📦)
  - Monster Editor (icon: 🐲)
  - Item Editor (icon: 🗡️)
  - Loot Table Editor (icon: 🎲)
  - Building Editor (icon: 🏰)
  - Procedural Rule Editor (icon: ⚙️)
  - Tilemap Tool (icon: 🗺️)
  - Animation Editor (icon: 🎞️)
  - Stat Curve Visualizer (icon: 📈)
  - Save/Export (icon: 💾)

- **Top Bar:**
  - Project name, file menu, quick actions, theme switcher

- **Main Panel:**
  - Displays the currently selected tool/editor
  - Responsive, resizable, with scroll areas as needed

- **Status Bar (bottom):**
  - Save status, notifications, tooltips

---

## Example: Monster Editor Panel
- **Header:** Monster Name, Type, Biome, Sprite Preview
- **Tabs:**
  - Stats (HP, ATK, DEF, Level, etc.)
  - Animation (Idle, Walk, Attack, Die; FPS, frame count)
  - Loot Table (linked, with preview)
  - AI/Behavior (profile selector)
- **Sidebar:** List of all monsters, search/filter
- **Main Area:** Editable fields, live preview, save/cancel buttons

---

## Example: Building Editor Panel
- **Header:** Building Name, Type, Tier, Biome Variant
- **Tabs:**
  - Footprint (tile grid, drag to resize)
  - Cost/HP/Defense (formulas, editable fields)
  - Production/Storage (type, output, capacity)
  - Upgrades (levels, bonuses)
- **Sidebar:** List of all buildings, search/filter
- **Main Area:** Modular part selector (roof, wall, door, etc.), sprite preview

---

## Example: Tilemap Tool
- **Toolbar:** Paint, Erase, Select, Fill, Place Building/Prop
- **Main Area:** Tilemap grid, drag & drop assets, zoom/pan
- **Sidebar:** Tile/asset palette, filter by type/biome

---

## Theming & Style
- **Dark mode by default** (deep blue/gray background, accent color highlights)
- **Consistent iconography** for all tools
- **Rounded panels, modern font, clear section headers**
- **Hover tooltips** for all buttons/fields

---

## Navigation & Shortcuts
- Sidebar icons for tool switching
- Ctrl+S to save, Ctrl+1..9 to switch tools
- Quick search (Ctrl+F) in asset lists

---

## Status Bar
- Shows: "Saved", "Unsaved changes", "Error: ...", "Tooltip: ..."

---

*This wireframe can be used as a reference for implementing the egui/ImGui UI in Rust or any other engine.*
