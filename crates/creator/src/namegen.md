# Fantasy RPG Name Generator

This module provides flexible, extensible name generation for all major fantasy RPG elements:
- Characters (heroes, NPCs)
- Towns and cities
- Dungeons and locations
- Items and artifacts
- Magic spells
- Kingdoms and regions
- Monsters and creatures

## Features
- Add, remove, and list fields and values for each name category
- Predefined generators for common RPG elements
- Easily extendable for custom categories
- Deterministic output with seedable RNG

## Example Usage

```rust
use creator::namegen::*;
use rand::thread_rng;

let mut rng = thread_rng();
let char_name = default_character_name_gen().generate(&mut rng);
let town_name = default_town_name_gen().generate(&mut rng);
let dungeon_name = default_dungeon_name_gen().generate(&mut rng);
let item_name = default_item_name_gen().generate(&mut rng);
let spell_name = default_spell_name_gen().generate(&mut rng);
let monster_name = default_monster_name_gen().generate(&mut rng);
let kingdom_name = default_kingdom_name_gen().generate(&mut rng);

println!("Character: {}", char_name);
println!("Town: {}", town_name);
println!("Dungeon: {}", dungeon_name);
println!("Item: {}", item_name);
println!("Spell: {}", spell_name);
println!("Monster: {}", monster_name);
println!("Kingdom: {}", kingdom_name);
```

## Extending
You can add new fields or values to any generator:

```rust
let mut gen = default_town_name_gen();
gen.add_value_to_field("prefix", "Crystal");
```

Or create a new category:

```rust
let mut custom = NameGenConfig::new(NameCategory::Custom("Deity".to_string()));
custom = custom.add_field("root", vec!["Astra", "Lumin", "Umbra"]);
```

See `namegen.rs` for full API.
