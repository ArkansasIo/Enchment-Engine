# Enchentment Engine Architecture

This document describes the high-level architecture of the Enchentment Engine, including its modular design, main components, and data flow.

## Overview
Enchentment Engine is a modular, cross-platform RPG engine built in Rust. It supports 2D, isometric, and first-person RPGs, with modern features like multiplayer and procedural content.

## Main Components
- **TheFramework**: Handles window creation, user event abstraction, and custom UI system.
- **Rusterix**: Core game engine, software rasterizer for 2D/3D geometry, in-game UI elements.
- **SceneVM**: Layer-based renderer built on wGPU, main rendering backbone.
- **Creator**: Main editor application for game development.
- **Client**: Game runtime for end-users.

## Data Flow
1. User interacts with the Creator UI.
2. Editor modifies game data and assets.
3. Engine modules process and render content.
4. Client loads and runs games using engine modules.

## Extensibility
- Modular crates allow for custom features and plugins.
- Open-source, MIT licensed.

---
For detailed module documentation, see the respective crate folders.
