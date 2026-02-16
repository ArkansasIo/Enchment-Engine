# Enchantment Engine Launcher & Installer Workspace

## Structure
- `launcher_api/` — Shared Rust library for launcher/installer logic
- `launcher/` — Launcher binary (uses launcher_api)
- `installer/` — Installer binary (uses launcher_api)
- `assets/` — Place splash, logo, and branding assets here
- `launcher_config.toml` — Config file for branding, installer/dev mode, etc.

## Build
From the `build/windows` directory, run:

```
cargo build --release
```

- This will build both the launcher and installer executables in their respective `target/release` folders.

## Usage
- Edit `launcher_config.toml` to customize branding and installer/dev mode.
- Run `launcher.exe` to start the engine (and optionally the installer).
- Installer is run automatically unless disabled in config or dev mode.

## Extending
- Add real GUI, error handling, or install logic as needed in the Rust code.
- See `launcher_api/src/lib.rs` for reusable API functions.

---
This workspace is ready for further extension and integration.
