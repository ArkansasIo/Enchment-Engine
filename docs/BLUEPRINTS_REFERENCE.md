# Unreal Engine Blueprints: Everything That Matters

## What is a Blueprint?
A Blueprint is a visual class/script asset in Unreal Engine. It can contain:
- **Graphs**: Event Graph (runtime logic), Function Graphs, Macro Graphs, Construction Script, Animation Graph, Material Graphs
- **Variables / Defaults**
- **Components** (for Actor Blueprints)
- **Interfaces implemented**
- **Dispatchers** (events you broadcast)
- **Timelines** (curve-driven updates)

## Pins: Inputs and Outputs
- **Exec pins (white wires)**: Control execution flow (Exec In/Out)
- **Data pins (colored wires)**: Carry values (input/output)
- **Pure nodes**: No exec pins, evaluate when outputs are needed
- **Type rules**: Types must match or be convertible

## Node Families (Everything You Use)
### A) Entry Points (Events)
- Event BeginPlay, Event Tick, Overlap, Input events, etc.

### B) Flow Control (Exec Logic)
- Branch, Sequence, Loops, Switch, Delay, Timers, Timeline, etc.

### C) Variables (State)
- Get/Set, local/member, primitives/structs/objects/arrays/maps/sets

### D) Functions
- Impure (exec pins), Pure (no exec pins, return values), Inputs/Outputs, Local vars

### E) Macros
- Inline-expanded logic, reusable graph snippets

### F) Communication
- Direct reference, Casting, Blueprint Interfaces, Event Dispatchers, Gameplay Framework references

### G) Actor Lifecycle
- SpawnActor, DestroyActor, Attach/Detach, Set Life Span, IsValid

### H) Components
- Add/Get/Transform components

### I) Math & Structs
- Numeric ops, Vector/Rotator/Transform ops, Break/Make Structs

### J) Collections
- Arrays, Maps, Sets (add/remove/find/foreach)

### K) Latent/Async Nodes
- Delay, Async Load, Timers, AI MoveTo, Line traces

### L) Debugging & Dev Tools
- Print String, Draw Debug, Breakpoints, Watch, Ensure, IsValid

## Input System
- **Legacy Input**: InputAxis, InputAction
- **Enhanced Input**: Input Action assets, Input Mapping Context, EnhancedInputComponent
- **Blueprint events**: Enhanced Input Action (Triggered/Ongoing/Completed)

## Outputs in Practice
- Function outputs (return values)
- Exec outputs (flow)
- Out parameters (by-ref outputs)

## Browsing the Full Node List
- Open any Blueprint graph, right-click, disable Context Sensitive for full palette
- Use search terms: Set/Get/Add/Remove, Trace, Timer, Interface, Dispatcher, Cast, Widget

## Node Categories
- Core Kismet libraries, Class functions, Plugin libraries

## Pin Color Cheat Sheet
- Bool: red
- Int: teal/greenish
- Float: yellow
- Name: mauve
- String: magenta
- Vector: yellow-green
- Rotator: blue
- Transform: orange
- Object ref: blue
- Exec: white

---
For the complete node list, always browse inside the Unreal Editor with Context Sensitive off.
