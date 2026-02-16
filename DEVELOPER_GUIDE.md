# Developer Guide

## Project Overview
This project is a modular, Unreal-style game engine written in Rust. It features core systems such as actors, components, world management, plugins, subsystems, and more.

## Getting Started
- Clone the repository
- Build with `cargo build`
- Run with `cargo run`

## Core Concepts
- **Actor**: Base object in the world, can have multiple components.
- **Component**: Logic/data attached to actors (e.g., TransformComponent).
- **World**: Manages all actors and updates them.
- **Subsystem**: Reusable engine service (e.g., logging, input).
- **Plugin**: Modular extension system for engine features.

## Map and World Systems

The engine includes a modular map system inspired by watabou.github.io generators:
- **MapLayout**: High-level map structure, holds zones.
- **World**: Collection of maps and global context.
- **Zone**: Regions/biomes within a map, each with its own grid.
- **Grid**: 2D grid for tiles, navigation, and terrain.
- **Coords**: Coordinate system for grid and map features.
- **Outline**: Shapes and borders for regions/zones.
- **Details**: Extra map features (labels, icons, etc).
- **Dungeon**: Procedural dungeon layout and rooms.
- **Location**: Named places (towns, dungeons, POIs).

You can extend these systems for procedural generation, import/export, or integration with external map tools.

## Watabou-style Map Engine

The engine includes a modular map system inspired by watabou.github.io generators, with dedicated modules for:
- **tools.rs**: Map editing, visualization, and analysis utilities (e.g., pathfinding, outlining, labeling, analysis).
- **logic.rs**: Core procedural generation logic for city and dungeon maps, and location placement.
- **import_export.rs**: Import/export support for map and dungeon data (e.g., JSON).

You can extend these modules to implement city/dungeon generation, pathfinding, map analysis, and integration with external map tools.

## Map Visualization: 2D and 3D Views

The engine supports both 2D and 3D map visualization:
- **view2d/**: 2D rendering, camera, grid, and overlays for classic top-down or tile-based maps.
- **view3d/**: 3D rendering, camera, grid, and overlays for terrain, world, or city visualization.

You can implement rendering logic in `renderer2d.rs` and `renderer3d.rs`, and connect map data (biomes, cities, kingdoms, etc.) to these views for interactive or procedural map displays.

## World, Kingdom, City, Biome, Building, and Place Systems

- **biome/**: Forest, desert, mountain, swamp, tundra, plains, and custom biomes.
- **building/**: House, inn, shop, castle, temple, and custom buildings.
- **place/**: Town, village, city, dungeon, landmark, and custom places.
- **kingdom/**: Realm, empire, duchy, principality, and custom kingdoms.
- **planet/**: World map, continent, ocean, climate, and custom planet-level data.

These modules are designed for extensibility and integration with procedural generation, import/export, and visualization tools.

## Usage Example: Map Generation

Here’s how to generate a basic map and dungeon using the engine’s map system:

```rust
use engine::map::generator::MapGenerator;

fn main() {
    let map = MapGenerator::generate_basic_map(16, 16);
    println!("Generated map with {} zones.", map.zones.len());
    let dungeon = MapGenerator::generate_dungeon(16, 16);
    println!("Generated dungeon with {} rooms.", dungeon.rooms.len());
}
```

See `engine/map/demo.rs` for a runnable demo.

## Adding New Features
1. Create a new module in the appropriate engine folder.
2. Implement the required trait or struct.
3. Register the module in the corresponding `mod.rs`.
4. Integrate with the world, actors, or plugin system as needed.

## Contributing
- Follow Rust best practices and formatting.
- Write integration tests for new features.
- Document new modules and public APIs.

## Roadmap
- Expand core components (render, input, etc.)
- Implement real subsystems (logging, input)
- Add integration tests and demos
- Set up CI/CD for automated builds
- Improve documentation and usage examples

## Contact
For questions or contributions, open an issue or pull request on GitHub.
