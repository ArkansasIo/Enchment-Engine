# Enchentment Engine

**Version:** 0.8.100  
**Developer:** stephen delinejr
**Website:** 

Enchentment Engine is a modular retro-style RPG/MMORPG game engine and editor written in Rust.

## Features
- 2D/3D editing workflows
- Unreal-style IDE layout (top/left/right toolbars, menus, windows)
- Town and fantasy world generation (continents, countries, capitals)
- RPG/MMORPG data generation and simulation
- Scripting, content pipelines, export stubs
- In-editor help and about dialogs

## Quick Start

```bash
cargo build -p enchantmen-creator
cargo run -p enchantmen-creator
```

## Main Documentation

- Docs Index: `docs/README.md`
- Architecture: `docs/ARCHITECTURE.md`
- IDE Layout and Toolbars: `docs/IDE_LAYOUT_AND_TOOLBARS.md`
- Town Generator: `docs/TOWN_GENERATOR_SYSTEM.md`
- Fantasy World Generator: `docs/FANTASY_WORLD_GENERATOR.md`
- RPG/MMORPG Systems: `docs/RPG_MMORPG_SYSTEMS.md`
- UML Docs: `docs/uml/README.md`

## RPG/MMORPG Features & Systems

- Character creation (race, class, appearance, stats)
- Leveling and experience system
- Skill trees and abilities
- Equipment and inventory management
- Turn-based and real-time combat systems
- Party management and AI companions
- Quest and story system (main, side, repeatable)
- Dialogue and branching choices
- Stat/attribute system (STR, DEX, INT, etc.)
- Status effects (buffs, debuffs, conditions)
- Loot tables and item drops
- Crafting, gathering, and professions
- World/zone/region/town/dungeon generation
- Map and minimap systems (2D/3D)
- Save/load and persistence

### MMORPG Features & Logic
- Account and character management (login, registration)
- Server-client networking (real-time sync, events)
- Instanced and persistent worlds
- Player trading and auction house
- Guilds, parties, and social systems (friends, chat, mail)
- PvP and PvE systems (arenas, battlegrounds, open world)
- World events and live content
- Group finder and matchmaking
- Mounts, pets, and companions
- Player housing and customization
- Economy and currency systems
- Anti-cheat and moderation tools
- Cross-server/world travel
- In-game mail and notification systems

### Advanced/Optional Systems
- Dynamic weather and day/night cycles
- Faction and reputation systems
- Achievements and leaderboards
- Cutscenes and cinematic tools
- Scripting and modding support
- AI for NPCs, monsters, and bosses
- Procedural content generation (maps, dungeons, quests)
- Voice chat and emotes
- Streaming/partitioning for large worlds (Unreal-style)
- Visual scripting (Blueprints) for gameplay logic

## License

MIT
