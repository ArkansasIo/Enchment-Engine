# Dragon Warrior–Style Engine Core Pillars

This document outlines the core systems, data schemas, and architecture for a classic Dragon Warrior (Dragon Quest)–style RPG engine.

---

## 1. World Structure
- **Overworld**: Tile map, grid movement, random encounters, warp tiles for towns/dungeons.
- **Towns/Dungeons**: Tile maps, no encounters in towns, dungeons have encounters, floors, stairs.
- **Battle Scene**: Separate mode/state from exploration.
- **GameState/Mode System**: MODE_TITLE, MODE_OVERWORLD, MODE_TOWN, MODE_DUNGEON, MODE_BATTLE, MODE_MENU, MODE_DIALOGUE, MODE_SHOP, MODE_INN, MODE_SAVE_LOAD.

## 2. Tile Map Engine
- **Map Model**: 2D grid of tile IDs (8×8 or 16×16 tiles).
- **Layers**: Ground, collision/property, event.
- **Tile Properties**: walkable, encounter_zone_id, terrain_type, move_cost, damage_per_step, warp_target, sfx_on_step.
- **Movement**: NES-style grid, direction updates, grid-aligned NPCs, interact with tile in front.

## 3. Random Encounter System
- **Trigger**: Each step on eligible tiles, roll RNG or use threat meter.
- **Variables**: encounter_rate, party_level, repel flag/timer.
- **Selection**: Encounter table per zone, weighted monster groups, formation rules.

## 4. Battle Engine (Turn-Based)
- **Phases**: Start, player command, resolve, enemy turn, end check.
- **Stats**: HP, MP, STR, AGI, DEF, (optional: LUCK).
- **Damage Formula**: attack = STR+weaponAtk, defense = DEF+armorDef, base = max(0, attack - defense/2), damage = rng_range(base*0.75, base*1.25), clamp >= 1.
- **Hit/Evade**: hitChance = clamp(AGI/(AGI+target.AGI), 0.1..0.95), RNG roll.
- **Criticals/Status**: Crits ×2, status effects (sleep, silence, poison, paralyze, buffs).
- **Enemy AI**: Weighted choice, context weights.

## 5. Menu & UI System
- **Modal UI Stack**: Push/pop layers, preserve world state.
- **Screens**: Main menu, dialogue, shop, inn, church, typewriter text, paging, tokens.

## 6. Inventory & Equipment
- **Inventory**: Limited slots, stack rules, item types (consumable, key, weapon, armor, spellbook).
- **Equipment**: Weapon, armor, shield, accessory. Affects stats/resistances.

## 7. Progression & Economy
- **Leveling**: Level table, required EXP, stat gains, spells learned.
- **Gold**: Enemy drops, shop pricing, inn cost scaling.

## 8. Event Scripting
- **Flags**: Global and per-map local flags.
- **Script Actions**: ShowText, SetFlag, GiveItem, GiveGold, StartBattle, Warp, PlaySfx, IfFlag.

## 9. Save System
- **Data**: Player stats, inventory, gold, map/position, flags, chest states, playtime/seed.

## 10. Engine Architecture
- **Modules**: GameStateManager, MapSystem, ActorSystem, EncounterSystem, BattleSystem, UISystem, DataSystem, SaveSystem, AudioSystem.
- **Data Tables**: Maps, Tiles, Enemies, EnemyGroups, EncounterZones, Items, Spells, LevelCurve.

---

## Folder Structure Example
- src/
  - main.rs
  - game_state.rs
  - map_system.rs
  - actor_system.rs
  - encounter_system.rs
  - battle_system.rs
  - ui/
    - menu.rs
    - dialogue.rs
    - shop.rs
  - data/
    - dt_maps.ron
    - dt_tiles.ron
    - dt_enemies.ron
    - dt_items.ron
    - dt_spells.ron
    - dt_level_curve.ron
  - save_system.rs
  - audio_system.rs

---

## Data Schema Example (RON)

### Map Tile
```ron
(
  id: 1,
  name: "Grass",
  walkable: true,
  encounter_zone_id: 1,
  terrain_type: "grass",
  move_cost: 1,
  damage_per_step: 0,
  warp_target: None,
  sfx_on_step: None,
)
```

### Enemy Group
```ron
(
  id: 1,
  name: "Slime Group",
  members: [ (enemy_id: 1, count: 2) ],
  formation: "line",
  weight: 10,
)
```

---

## State Machine Example
- Title → Overworld → (Town/Dungeon) → Battle → Menu/Dialog → Save/Load

---

## Battle Formula Example
```
attack = STR + weaponAtk
defense = DEF + armorDef
base = max(0, attack - defense/2)
damage = rng_range(base*0.75, base*1.25)
if hit: clamp >= 1
```

---

This structure provides a robust, modular foundation for a Dragon Warrior–style RPG engine.
