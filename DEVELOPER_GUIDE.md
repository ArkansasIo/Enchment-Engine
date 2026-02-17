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

# D&D 5e Resource Reference

## Races
| Race        | Traits/Features                        | Subraces           |
|-------------|----------------------------------------|--------------------|
| Human       | +1 all abilities, extra language       | Variant Human      |
| Elf         | Darkvision, Keen Senses, Fey Ancestry  | High, Wood, Drow   |
| Dwarf       | Darkvision, Dwarven Resilience         | Hill, Mountain     |
| Halfling    | Lucky, Brave, Halfling Nimbleness      | Lightfoot, Stout   |
| Dragonborn  | Draconic Ancestry, Breath Weapon       | —                  |
| Gnome       | Darkvision, Gnome Cunning              | Forest, Rock       |
| Half-Elf    | Darkvision, Fey Ancestry, Versatility  | —                  |
| Half-Orc    | Darkvision, Relentless Endurance       | —                  |
| Tiefling    | Darkvision, Hellish Resistance         | —                  |
| Aasimar     | Protector, Scourge, Fallen             | —                  |
| Genasi      | Air, Earth, Fire, Water                | —                  |
| Goliath     | —                                    | —                  |
| Tabaxi      | —                                    | —                  |
| Firbolg     | —                                    | —                  |
| Kenku       | —                                    | —                  |
| Lizardfolk  | —                                    | —                  |
| Triton      | —                                    | —                  |
| Yuan-ti Pureblood | —                                    | —                  |
| Bugbear     | —                                    | —                  |
| Goblin       | —                                    | —                  |
| Hobgoblin    | —                                    | —                  |
| Kobold       | —                                    | —                  |
| Orc          | —                                    | —                  |
| Tortle       | —                                    | —                  |
| Leonin       | —                                    | —                  |
| Satyr        | —                                    | —                  |
| Minotaur     | —                                    | —                  |
| Centaur      | —                                    | —                  |
| Loxodon      | —                                    | —                  |
| Simic Hybrid | —                                    | —                  |
| Vedalken     | —                                    | —                  |
| Warforged    | —                                    | —                  |
| Changeling   | —                                    | —                  |
| Kalashtar    | —                                    | —                  |
| Shifter      | —                                    | —                  |
| Aarakocra    | —                                    | —                  |
| Locathah     | —                                    | —                  |
| Grung         | —                                    | —                  |
| Verdan        | —                                    | —                  |

## Classes
| Class      | Hit Die | Primary Abilities | Spellcasting | Example Subclasses         |
|------------|---------|-------------------|--------------|---------------------------|
| Barbarian  | d12     | STR, CON          | No           | Berserker, Totem Warrior  |
| Bard       | d8      | CHA, DEX          | Yes          | Lore, Valor               |
| Cleric     | d8      | WIS, STR/DEX      | Yes          | Life, Light, Trickery     |
| Druid      | d8      | WIS, CON          | Yes          | Land, Moon                |
| Fighter    | d10     | STR/DEX, CON      | No           | Champion, Battlemaster    |
| Monk       | d8      | DEX, WIS          | No           | Open Hand, Shadow         |
| Paladin    | d10     | STR, CHA          | Yes          | Devotion, Vengeance       |
| Ranger     | d10     | DEX, WIS          | Yes          | Hunter, Beast Master      |
| Rogue      | d8      | DEX, INT          | No           | Thief, Assassin           |
| Sorcerer   | d6      | CHA, CON          | Yes          | Draconic, Wild Magic      |
| Warlock    | d8      | CHA, CON          | Yes          | Fiend, Archfey            |
| Wizard     | d6      | INT, DEX          | Yes          | Evocation, Illusion       |
| Artificer  | d8      | INT, CON          | Yes          | Alchemist, Artillerist    |

## Backgrounds
| Background | Skills/Tools/Languages | Feature Example         |
|------------|-----------------------|------------------------|
| Acolyte    | Insight, Religion     | Shelter of the Faithful|
| Criminal   | Deception, Stealth    | Criminal Contact       |
| Folk Hero  | Animal Handling, Survival | Rustic Hospitality |
| Noble      | History, Persuasion   | Position of Privilege  |
| Sage       | Arcana, History       | Researcher             |
| Soldier    | Athletics, Intimidation | Military Rank        |

## Abilities & Skills
| Ability      | Related Skills                        |
|--------------|--------------------------------------|
| Strength     | Athletics                            |
| Dexterity    | Acrobatics, Sleight of Hand, Stealth |
| Constitution | —                                    |
| Intelligence | Arcana, History, Investigation, Nature, Religion |
| Wisdom       | Animal Handling, Insight, Medicine, Perception, Survival |
| Charisma     | Deception, Intimidation, Performance, Persuasion |

## Equipment
### Weapons
| Type    | Examples                |
|---------|-------------------------|
| Simple  | Club, Dagger, Mace      |
| Martial | Longsword, Greatsword   |
| Ranged  | Shortbow, Longbow       |

### Armor
| Type    | Examples                |
|---------|-------------------------|
| Light   | Leather, Studded Leather|
| Medium  | Chain Shirt, Scale Mail |
| Heavy   | Plate, Chain Mail       |
| Shield  | Shield                  |

### Adventuring Gear
| Examples                |
|-------------------------|
| Rope, Rations, Thieves' Tools, Healer's Kit |

## Spells
- Spell levels: 0 (Cantrips) to 9
- Schools: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation
- Example spells: Fireball, Cure Wounds, Mage Hand, Shield, Eldritch Blast

## Monsters & NPCs
| Stat Block Fields      |
|-----------------------|
| Name, Type, Size, Alignment, AC, HP, Speed, Abilities, Skills, Senses, Languages, Challenge Rating, Traits, Actions, Reactions, Legendary/Lair Actions, Regional Effects |

## Rules & Mechanics
- Proficiency bonus, advantage/disadvantage, saving throws, conditions (blinded, charmed, etc.), initiative, combat order, movement, actions, reactions, bonus actions.

## D&D 5e Full Reference Tables

### Races (PHB + more)
- Human
- Elf (High, Wood, Drow, Eladrin, Sea, Shadar-kai)
- Dwarf (Hill, Mountain, Duergar)
- Halfling (Lightfoot, Stout, Ghostwise)
- Dragonborn (Chromatic, Metallic, Gem)
- Gnome (Forest, Rock, Deep/Svirfneblin)
- Half-Elf
- Half-Orc
- Tiefling (Asmodeus, Zariel, etc.)
- Aasimar (Protector, Scourge, Fallen)
- Genasi (Air, Earth, Fire, Water)
- Goliath
- Tabaxi
- Firbolg
- Kenku
- Lizardfolk
- Triton
- Yuan-ti Pureblood
- Bugbear
- Goblin
- Hobgoblin
- Kobold
- Orc
- Tortle
- Leonin
- Satyr
- Minotaur
- Centaur
- Loxodon
- Simic Hybrid
- Vedalken
- Warforged
- Changeling
- Kalashtar
- Shifter
- Aarakocra
- Locathah
- Grung
- Verdan

### Classes & Subclasses (PHB + Tasha's/Xanathar's)
- Barbarian: Berserker, Totem Warrior, Ancestral Guardian, Storm Herald, Zealot, Wild Magic, Beast
- Bard: Lore, Valor, Glamour, Swords, Whispers, Eloquence, Creation
- Cleric: Knowledge, Life, Light, Nature, Tempest, Trickery, War, Arcana, Forge, Grave, Order, Peace, Twilight
- Druid: Land, Moon, Dreams, Shepherd, Spores, Stars, Wildfire
- Fighter: Champion, Battlemaster, Eldritch Knight, Arcane Archer, Cavalier, Samurai, Psi Warrior, Rune Knight, Echo Knight
- Monk: Open Hand, Shadow, Four Elements, Drunken Master, Kensei, Sun Soul, Astral Self, Mercy
- Paladin: Devotion, Ancients, Vengeance, Conquest, Crown, Glory, Redemption, Oathbreaker, Watchers
- Ranger: Hunter, Beast Master, Gloom Stalker, Horizon Walker, Monster Slayer, Fey Wanderer, Swarmkeeper
- Rogue: Thief, Assassin, Arcane Trickster, Inquisitive, Mastermind, Scout, Swashbuckler, Phantom, Soulknife
- Sorcerer: Draconic, Wild Magic, Divine Soul, Shadow, Storm, Aberrant Mind, Clockwork Soul
- Warlock: Archfey, Fiend, Great Old One, Celestial, Hexblade, Fathomless, Genie, Undead, Undying
- Wizard: Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, Bladesinging, War Magic, Scribes
- Artificer: Alchemist, Artillerist, Battle Smith, Armorer

### Backgrounds (PHB + more)
- Acolyte, Anthropologist, Archaeologist, Charlatan, City Watch, Clan Crafter, Cloistered Scholar, Courtier, Criminal, Entertainer, Faction Agent, Far Traveler, Folk Hero, Gladiator, Guild Artisan, Guild Merchant, Hermit, Inheritor, Investigator, Knight, Knight of the Order, Mercenary Veteran, Noble, Outlander, Pirate, Sage, Sailor, Soldier, Spy, Urban Bounty Hunter, Urchin, Waterdhavian Noble

### Skills (All)
- Acrobatics (Dex)
- Animal Handling (Wis)
- Arcana (Int)
- Athletics (Str)
- Deception (Cha)
- History (Int)
- Insight (Wis)
- Intimidation (Cha)
- Investigation (Int)
- Medicine (Wis)
- Nature (Int)
- Perception (Wis)
- Performance (Cha)
- Persuasion (Cha)
- Religion (Int)
- Sleight of Hand (Dex)
- Stealth (Dex)
- Survival (Wis)

### Weapons (All PHB)
- Simple Melee: Club, Dagger, Greatclub, Handaxe, Javelin, Light Hammer, Mace, Quarterstaff, Sickle, Spear
- Simple Ranged: Light Crossbow, Dart, Shortbow, Sling
- Martial Melee: Battleaxe, Flail, Glaive, Greataxe, Greatsword, Halberd, Lance, Longsword, Maul, Morningstar, Pike, Rapier, Scimitar, Shortsword, Trident, War Pick, Warhammer, Whip
- Martial Ranged: Blowgun, Hand Crossbow, Heavy Crossbow, Longbow, Net

### Armor (All PHB)
- Light: Padded, Leather, Studded Leather
- Medium: Hide, Chain Shirt, Scale Mail, Breastplate, Half Plate
- Heavy: Ring Mail, Chain Mail, Splint, Plate
- Shields

### Spell Schools
- Abjuration
- Conjuration
- Divination
- Enchantment
- Evocation
- Illusion
- Necromancy
- Transmutation

### Monster/NPC Stat Block Fields
- Name, Size, Type, Alignment, Armor Class, Hit Points, Speed, Abilities (Str, Dex, Con, Int, Wis, Cha), Saving Throws, Skills, Damage Resistances, Damage Immunities, Condition Immunities, Senses, Languages, Challenge Rating, Traits, Actions, Reactions, Legendary Actions, Lair Actions, Regional Effects

### Core Mechanics
- Proficiency Bonus, Advantage/Disadvantage, Initiative, Movement, Actions, Bonus Actions, Reactions, Attack Rolls, Saving Throws, Skill Checks, Spellcasting, Concentration, Resting, Leveling, Multiclassing, Feats, Conditions (Blinded, Charmed, Deafened, Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned, Unconscious, Exhaustion)

### Reference Links
- [D&D 5e SRD](https://dnd5e.info/)
- [Open5e API](https://api.open5e.com/)
- [Roll20 Compendium](https://roll20.net/compendium/dnd5e/BookIndex)
- [5eTools](https://5e.tools/)
