# Town Generator System

This engine now includes a procedural town-generation subsystem inspired by:

- `watabou/TownGeneratorOS`  
  <https://github.com/watabou/TownGeneratorOS>

## What was integrated

- A native Rust town-generation pipeline in `creator/src/game_logic/mod.rs`
- District/ward modeling (`WardType`, `DistrictNode`)
- Road graph generation (`Road`)
- Landmark and gate extraction (`Landmark`, `gates`)
- Seeded deterministic output (`TownGeneratorSettings`)
- Editor integration:
  - Tools menu entry: `Generate Town (Watabou-style)`
  - Runtime generation and embedding into project config
- RPG/MMORPG systems integration:
  - Native systems in `creator/src/game_logic/rpg_mmorpg.rs`
  - Tools menu entry: `Generate RPG/MMORPG Systems`
  - Runtime generation and embedding into project config key `[mmorpg_systems]`

## How to use

1. Open the Creator app.
2. Use `Tools -> Town Size Presets` and pick:
   - `Small Town`
   - `Large Town`
   - `Small City`
   - `Large City`
3. Optional toggles:
   - `Tools -> Town: Toggle River`
   - `Tools -> Town: Toggle Walls`
4. Generate:
   - `Tools -> Town: Reseed + Generate` (new seed)
   - `Tools -> Town: Regenerate Same Seed` (same seed)
   - or `Tools -> Generate Town (Watabou-style)`
3. The generated result is stored in `project.config` under `[town_generator]`.

## Stored config keys

- `source`
- `source_repo`
- `seed`
- `district_count`
- `road_count`
- `landmark_count`
- `last_generated` (JSON payload)

## Notes

- This is an engine-adapted rewrite, not a direct Haxe runtime import.
- It is designed to be expanded into region/map baking and gameplay spawning later.
