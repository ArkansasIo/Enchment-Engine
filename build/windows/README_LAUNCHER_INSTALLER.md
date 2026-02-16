# Enchantment Engine Launcher & Installer

## Overview
- **Launcher**: Full-featured app with splash, logo, loading bar, config-driven installer/dev mode, and modular API (see api/launcher_api.rs).
- **Installer**: Full-featured app with splash, logo, loading bar, modular API (see api/installer_api.rs).

## Configuration
Edit `launcher_config.toml` to control installer, dev mode, and asset paths:

```
enable_installer = true
dev_mode = false
splash_screen = "../../enchantment_engine_branding_pack/splash_screens/splash_1920x1080.png"
logo = "../../enchantment_engine_branding_pack/launcher/launcher_icon_1024.png"
loading_bar_color = "#FFD740"
loading_bar_background = "#222222"
engine_exe = "../enchantmen-creator.exe"
```

- Set `enable_installer = false` to skip installer (dev mode).
- Set `dev_mode = true` for development (installer always off).

## Building
- Build API modules as libraries or copy into your project.
- Build launcher: `rustc main.rs -L ../api -o launcher.exe` (from launcher_app)
- Build installer: `rustc main.rs -L ../api -o installer.exe` (from installer_app)

## Usage
- Run `launcher.exe` to start the engine (and optionally the installer).
- Installer is run automatically unless disabled in config or dev mode.

## Customization
- Replace splash/logo images in `enchantment_engine_branding_pack` as needed.
- Adjust loading bar colors in config.
- Extend API modules for GUI, error handling, or real install logic.

## API Reference
See `api/README_API.md` for details on the modular API.

---
This system is modular and ready for further extension (GUI, real install logic, etc.).
