# Engine Systems

## Overview

Enchentment Engine is split into modular editor and runtime systems.

## Main Modules

- `creator/src/editor.rs`
Editor shell, menus, toolbars, dialogs, workspace persistence, high-level commands.

- `creator/src/game_logic/mod.rs`
Town generation and fantasy world generation logic.

- `creator/src/game_logic/rpg_mmorpg.rs`
RPG/MMORPG domain models, config builders, progression/combat/loot logic.

- `creator/src/tools/*`
Interactive editor tools (selection, geometry, world, render, entity, data, config, info).

- `creator/src/docks/*`
Docked panel modules (tiles, code, visual code, data, log, tilemap).

## Data Persistence

- Workspace layout and options are stored in project config TOML under `[editor_layout]`.
- Generated systems are serialized into project config sections:
- `[town_generator]`
- `[fantasy_world_generator]`
- `[mmorpg_systems]`

## Event Handling

UI interaction routes through editor event handlers:
- `ContextMenuSelected`
- `StateChanged`
- `ValueChanged`
- `IndexChanged`

This keeps menu actions, toolbar actions, and panel buttons consistent.
