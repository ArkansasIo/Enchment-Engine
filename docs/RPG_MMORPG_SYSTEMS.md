# RPG/MMORPG Systems

## Purpose

Provide a configurable builder for RPG/MMORPG foundations:
- Class and race templates
- Skill tiers
- Quest pools
- Loot table
- World runtime settings

## Core Models

Defined in `creator/src/game_logic/rpg_mmorpg.rs`:
- `RpgMmorpgCreateInput`
- `CharacterClass`
- `CharacterArchetype`
- `RaceTemplate`
- `ClassTemplate`
- `StarterRpgMmorpgConfig`

Runtime helpers include:
- XP progression (`apply_xp`)
- Combat resolution (`resolve_combat_event`)
- Loot rolls (`roll_loot`)
- World ticking (`tick_world`)
- Party/Guild helpers

## Builder Inputs (Editor)

- World name
- Max players per shard
- Starting level
- Race type count
- Quest count
- Skill tier count
- Included classes (warrior/ranger/mage/cleric/rogue)
- XP/loot/event rates

## Output

Serialized into project config:
- `[mmorpg_systems]`
- Summary counters + JSON `data`

## Simulation

UI supports simulation actions:
- Tick
- Combat
- Loot

These actions use generated config and scale by the configured rates.
