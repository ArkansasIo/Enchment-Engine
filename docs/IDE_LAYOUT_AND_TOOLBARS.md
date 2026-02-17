# IDE Layout and Toolbars

## Layout Model

The editor uses an Unreal-style workspace:
- Top main menu + top quick tools
- Left mode/tools panel
- Right quick actions + settings panel
- Center viewport/editor stack
- Bottom status/dock area

## Top Menus

- `File`: New/Open/Save/Save As/Close
- `Edit`: Undo/Redo/Copy/Paste
- `View`: Left/Right panel toggles, theme presets
- `Tools`: 2D tools, 3D tools, town systems, fantasy world generation, RPG/MMORPG systems
  - Includes `Editor Tools` and `All Tools` submenus so every registered tool can be selected from the menu API
- `Build`: Play/Pause/Stop, export stubs
- `Settings`: snap/grid/gizmos, IDE layout presets, feature matrix
- `Window`: Content Browser, World Outliner, Details, Output Log, Blueprint
- `Help`: docs/examples/about

## Left Toolbar

Grouped sections:
- Modes
- 2D
- 3D
- Editor
- Actions

Actions are linked to the same command IDs as top menu actions, so behavior is shared.

## Right Toolbar and Settings

Right quick actions contain icon + text labels for visibility.

Settings include:
- Theme and viewport options
- Town generation controls
- Fantasy world generator inputs
- RPG/MMORPG builder inputs
- Simulation controls

## Presets

Available IDE layout presets:
- Unreal-like
- Minimal
