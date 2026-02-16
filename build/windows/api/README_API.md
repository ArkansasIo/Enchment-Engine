# Launcher & Installer API

This folder contains modular Rust APIs for the launcher and installer apps.

## launcher_api.rs
- `LauncherConfig`: Loads config from TOML.
- `show_splash`, `show_logo`, `show_loading_bar`: Display branding and progress.
- `run_installer`: Launches the installer app.
- `launch_engine`: Launches the main engine executable.

## installer_api.rs
- `InstallerConfig`: Loads config from TOML.
- `show_splash`, `show_logo`, `show_loading_bar`: Display branding and progress.
- `perform_install`: Simulates installation steps.

## Usage
- Import these modules in your launcher/installer main.rs for a modular, maintainable codebase.
- Extend with GUI, error handling, or real install logic as needed.

---
For a full GUI, integrate with a Rust GUI framework (e.g., egui, druid, fltk, etc.).
