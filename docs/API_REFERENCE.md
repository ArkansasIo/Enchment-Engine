# API Reference: Enchentment Engine

## Modules
- `engine::keybinds` — Key binding management
- `engine::macros` — Macro system
- `engine::assets` — Asset library
- `engine::ai` — AI systems (FSM, planner)
- `engine::map` — Map and world systems
- `engine::core` — Core engine utilities

## Example: Keybinds
```rust
let mut kb = engine::keybinds::Keybinds::new();
kb.bind("Jump", "Space");
```

## Example: Macros
```rust
let mut ms = engine::macros::MacroSystem::new();
ms.add_macro("QuickSave", vec!["Save".to_string()]);
```

See each module's `mod.rs` for more details and usage.
