# Enchentment Engine UML Diagrams

This document provides UML diagrams for the main modules and their interactions.

## Module Overview
- TheFramework
- Rusterix
- SceneVM
- Creator
- Client

## Example UML: Engine Module Relationships

```
classDiagram
    class TheFramework {
        +window_creation()
        +event_abstraction()
        +ui_system()
    }
    class Rusterix {
        +rasterize()
        +game_logic()
    }
    class SceneVM {
        +render_layers()
        +gpu_rendering()
    }
    class Creator {
        +edit_game()
        +manage_assets()
    }
    class Client {
        +run_game()
    }
    TheFramework <--> Rusterix
    Rusterix <--> SceneVM
    Creator <--> TheFramework
    Client <--> SceneVM
```

---
For more detailed diagrams, see the source code and architecture documentation.
