# System Requirements Specification (SRS)

## Scope

Enchentment Engine is a modular editor and runtime toolkit for building RPG/MMORPG games with 2D/3D workflows and procedural generation systems.

## Functional Requirements

- FR-01: Editor shall provide menu-driven commands for project lifecycle (new/open/save/export).
- FR-02: Editor shall provide top, left, and right toolbar interactions linked to the same command handlers.
- FR-03: Editor shall provide town generation with preset, seed, overlays, and map bake.
- FR-04: Editor shall provide fantasy world generation with continents/countries/capital towns.
- FR-05: Editor shall provide RPG/MMORPG builder inputs and generation.
- FR-06: System shall serialize generator outputs into project TOML config sections.
- FR-07: Runtime tools shall support simulation actions (tick/combat/loot).
- FR-08: Workspace settings shall persist and reload per project.

## Non-Functional Requirements

- NFR-01: Deterministic generation for identical seeds/settings.
- NFR-02: Cross-platform operation (Windows/macOS/Linux).
- NFR-03: Modular source organization by subsystem.
- NFR-04: Undo/redo support for core map editing workflows.
- NFR-05: UI responsiveness sufficient for interactive editing.

## Data and Configuration

- Project config format: TOML
- Major sections:
- `editor_layout`
- `town_generator`
- `fantasy_world_generator`
- `mmorpg_systems`

## Verification

- Unit tests for deterministic generation and model integrity.
- Build validation with `cargo check`.
- Runtime validation through interactive editor actions.
