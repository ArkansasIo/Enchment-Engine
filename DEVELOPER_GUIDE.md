# Developer Guide: Enchentment Engine

## Building the Source Code

1. **Install Rust**
   - Download and install Rust from [https://rustup.rs](https://rustup.rs)
   - Ensure `cargo` is available in your terminal: `cargo --version`

2. **Clone the Repository**
   ```sh
   git clone https://github.com/ArkansasIo/Enchment-Engine.git
   cd Enchment-Engine
   ```

3. **Build the Engine**
   ```sh
   cargo build --release
   ```
   - Binaries will be in `target/release/`

4. **Run Tests**
   ```sh
   cargo test --all --tests
   ```

5. **Build Launcher/Installer (Windows)**
   ```sh
   cd build/windows
   cargo build --release
   ```
   - See `build/windows/README.md` for details.

## Using the Game Engine

- **Run the Demo Game**
  ```sh
  cargo run --example demo_game
  ```
  - See `examples/demo_game/main.rs` for usage of keybinds, macros, and game loop.

- **Add Your Own Game**
  1. Create a new file in `examples/` (e.g., `my_game.rs`).
  2. Use engine modules: `use engine::keybinds::*;`, `use engine::macros::*;`, etc.
  3. Implement your game logic and run with:
     ```sh
     cargo run --example my_game
     ```

- **Asset Management**
  - Use `engine::assets::AssetLibrary` to register and load images, sounds, and data.

- **Custom Controls & Macros**
  - Use `engine::keybinds` for custom key mapping.
  - Use `engine::macros` to automate actions.

## Contributing
- See `CONTRIBUTING.md` for code style and pull request guidelines.

---
For more details, see the README and code comments throughout the project.
