# Plugin Folder Architecture

- Each plugin is a folder under `plugins/` with its own `Cargo.toml` and `src/`.
- All plugins implement the `Plugin` trait (see `plugin_manager.rs`).
- Plugins are loaded and registered via `PluginManager` at runtime.
- Plugins can communicate via shared events, service locators, or direct API calls if exposed.

## Example Structure

plugins/
  core_runtime/
    Cargo.toml
    src/
      lib.rs
  renderer_core/
    Cargo.toml
    src/
      lib.rs
  ...
  plugin_manager.rs
  mod.rs

## Communication & Registration
- Plugins call `PluginManager::load_plugin(Box::new(MyPlugin))` to register.
- Use the `register()` and `init()` methods for setup and runtime logic.
- For cross-plugin communication, use event bus, shared state, or dependency injection patterns.
