# MapForge Integration Guide

This folder contains the MapForge procedural town generator, ported for use with the Enchentment Engine.

- Use `bulk_copy_mapforge.ps1` in the project root to update all MapForge files from the original source.
- The engine can launch MapForge as a separate process.

Example:

```rust
Command::new("MapForge.exe").spawn().expect("Failed to launch MapForge");
```

- Add export logic to MapForge (Haxe/OpenFL) to save the generated map as PNG/SVG.
- Add a build script or cargo task to build MapForge (Haxe/OpenFL) as part of your engine's build pipeline.

Example (Rust):

```rust
let img = image::open("MapForge/Assets/town.png").unwrap();
```
