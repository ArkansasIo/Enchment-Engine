# Architecture Overview: Enchentment Engine

## Structure
- Modular Rust engine with plugin-based systems
- Major modules: core, ai, map, assets, keybinds, macros
- Example/demo games in `examples/`

## Key Concepts
- Plugin registration and event system
- Extensible asset and input management
- Test suite for controls, macros, and payment

## Extending
- Add new modules in `engine/`
- Register in `engine/mod.rs`
- Add tests in `tests/game_dev_suite/`

See DEVELOPER_GUIDE.md for build and extension instructions.
