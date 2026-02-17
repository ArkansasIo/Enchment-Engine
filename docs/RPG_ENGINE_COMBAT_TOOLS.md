# Combat System Tools Documentation

This section describes the tools and systems for designing, balancing, and simulating combat in an RPG engine.

## Combat Framework
- **Damage & Mitigation Formulas**: Define how attacks, spells, and abilities deal and reduce damage. Includes resistances, armor, and vulnerabilities.
- **Hit Detection & Targeting**: Systems for determining if attacks hit, miss, or crit. Supports area-of-effect, line, and cone targeting.
- **Ability & Skill Execution**: Executes character abilities, spells, and skills, including animations, effects, and cooldowns.
- **Cooldown & Resource Costs**: Manages ability cooldowns, mana/energy costs, and resource regeneration.
- **Threat / Aggro System**: Tracks which entities are targeted by enemies based on actions, damage, and proximity.

## Combat Modes
- **Turn-based**: Classic RPG combat with initiative, rounds, and action points.
- **Real-time with Pause**: Allows pausing to issue commands, then resumes real-time action.
- **Action Combat**: Direct control, dodging, and timing-based attacks.
- **Tactical Grid Combat**: Movement and attacks occur on a grid, supporting positioning and cover.

## Best Practices
- Make combat rules data-driven for easy balancing.
- Support both PvE and PvP combat scenarios.
- Provide debugging and visualization tools for combat events and logs.

---

Next: Inventory & Item Systems
