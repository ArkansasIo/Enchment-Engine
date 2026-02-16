
# Enchentment Engine: Next-Generation Classical RPG Creator
![Enchentment Engine Header](images/enchantmen_header.png)
---
![Windows]
## Overview
**Enchentment Engine** is a cross-platform, open-source creator for classic retro role-playing games (RPGs). It empowers you to build 2D, isometric, and first-person RPGs reminiscent of the 1980s and 1990s, while supporting modern features like multiplayer, procedural content, and more.

- **Multi-platform:** Windows, macOS, Linux
- **Flexible rendering:** 2D, isometric, and 3D
- **Modern features:** Multiplayer, procedural generation, modular engine
- **Open-source:** MIT License
## Screenshots

2D Example           | 3D Example
:-------------------------:|:-------------------------:
![Enchentment Engine Screenshot](images/hideout2d.png)  |  ![Enchentment Engine Screenshot](images/dungeon3d.png)

## Architecture

Enchentment Engine is built on a modular Rust codebase:

- **TheFramework:** Cross-platform windowing, UI, and event abstraction
- **Rusterix:** Core game engine, software rasterization, and in-game UI
- **SceneVM:** Layer-based 2D/3D renderer built on [wGPU](https://wgpu.rs)
- **Features & Game Logic:** Easily extendable via `src/features/` and `src/game_logic/`

## Getting Started

### Install via Cargo
If you have [Rust installed](https://www.rust-lang.org/tools/install):

```bash
cargo install enchentment_engine-creator
cargo install enchentment_engine-client
```

### Linux Dependencies
Install: `libasound2-dev` `libatk1.0-dev` `libgtk-3-dev`

## Contributing
- Add new features in `src/features/` and game logic in `src/game_logic/`
- See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines

## License
MIT License. All assets are MIT unless otherwise stated.

## Support
- [Patreon]
- [Donate](
---

For more info, visit [enchentmentengine.com](https://enchentmentengine.com)
#
# (This project was formerly known as Eldiron)

---

![Windows](

**Enchentment Engine** (formerly Eldiron) is a cross-platform creator for classic retro role-playing games (RPGs). Its primary goal is to enable the creation of RPGs reminiscent of the 1980s and 1990s while incorporating modern features such as multiplayer support, procedural content generation, and more.


Enchentment Engine natively supports **2D** (like Ultima 4/5), **isometric**, and **first-person** RPGs, allowing developers to craft a variety of experiences effortlessly.


Enchentment Engine is open-source and licensed under the **MIT License**.

For a more detailed feature overview please visit [

2D Example           | 3D Example
:-------------------------:|:-------------------------:
![Enchentment Engine Screenshot](images/hideout2d.png)  |  ![Enchentment Engine Screenshot](images/dungeon3d.png)

## Source Code


Enchentment Engine is built on three embedded crates that I have developed over the last years. Each focuses on a specific aspect of the engine and editor, and together they form the foundation of the Enchentment Engine ecosystem.

- **TheFramework**
  Handles cross-platform window creation, user event abstraction, and the custom UI system used by *Enchentment Engine Creator*.

- **Rusterix**
  Started as a software rasterizer for 2D and 3D geometry, but has since evolved into the core game engine. While *SceneVM* now handles most rendering tasks, the software rasterization aspect of Rusterix is still used for example in in-game UI elements.

- **SceneVM**
  An abstracted, layer-based renderer for 2D and 3D built on top of [wGPU](https://wgpu.rs). Each layer can define its own geometry and compute shaders, making SceneVM the main rendering backbone of engine 


Over the past five years, Enchentment Engine (formerly Eldiron) has gone through several major iterations. As a result, some parts of the code are in the process of being consolidated or phased out as the project moves toward a cleaner v1 architecture.

---

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [GitHub Releases]

### Install via Cargo

If you have [Rust installed](https://www.rust-lang.org/tools/install), you can install Enchentment Engine Creator and the Client directly from [crates.io](https://crates.io):

```bash
cargo install enchentment_engine-creator
cargo install enchentment_engine-client
```

### Building from Source

Clone the repository and build:

```bash
git clone 
cd enchentment_engine
cargo run --release --package creator
```

### Linux Dependencies

Make sure these dependencies are installed: `libasound2-dev` `libatk1.0-dev` `libgtk-3-dev`

## License


The source and all assets I commissioned for rpg mmorpg engine are licensed under the MIT.

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in Enchentment Engine, shall be MIT licensed as above, without any additional terms or conditions.

---

## Sponsor

