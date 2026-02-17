// Fantasy RPG Name Generator Core Structures
// Supports: characters, towns, dungeons, items, magic, etc.
// Add more categories as needed

use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug, Clone)]
pub enum NameCategory {
    Character,
    Town,
    Dungeon,
    Item,
    MagicSpell,
    Kingdom,
    Forest,
    Artifact,
    Monster,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct NameGenField {
    pub key: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NameGenConfig {
    pub category: NameCategory,
    pub fields: Vec<NameGenField>,
}

impl NameGenConfig {
    pub fn new(category: NameCategory) -> Self {
        Self { category, fields: vec![] }
    }
    pub fn add_field(mut self, key: &str, values: Vec<&str>) -> Self {
        self.fields.push(NameGenField {
            key: key.to_string(),
            values: values.into_iter().map(|s| s.to_string()).collect(),
        });
        self
    }
    pub fn generate<R: Rng>(&self, rng: &mut R) -> String {
        let mut parts = vec![];
        for field in &self.fields {
            if let Some(val) = field.values.choose(rng) {
                parts.push(val.clone());
            }
        }
        parts.join(" ")
    }

    /// Add a value to an existing field or create the field if it doesn't exist
    pub fn add_value_to_field(&mut self, key: &str, value: &str) {
        if let Some(field) = self.fields.iter_mut().find(|f| f.key == key) {
            field.values.push(value.to_string());
        } else {
            self.fields.push(NameGenField {
                key: key.to_string(),
                values: vec![value.to_string()],
            });
        }
    }

    /// Remove a value from a field
    pub fn remove_value_from_field(&mut self, key: &str, value: &str) {
        if let Some(field) = self.fields.iter_mut().find(|f| f.key == key) {
            field.values.retain(|v| v != value);
        }
    }

    /// Get all values for a field
    pub fn get_field_values(&self, key: &str) -> Option<&[String]> {
        self.fields.iter().find(|f| f.key == key).map(|f| f.values.as_slice())
    }

    /// List all field keys
    pub fn field_keys(&self) -> Vec<&str> {
        self.fields.iter().map(|f| f.key.as_str()).collect()
    }
}

// Example: Predefined generators for common RPG elements
pub fn default_character_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Character)
        .add_field("prefix", vec!["Al", "El", "Mor", "Ser", "Val", "Gal", "Tor", "Fen"])
        .add_field("root", vec!["dric", "lian", "mir", "thas", "gorn", "wyn", "dell", "rion"])
        .add_field("suffix", vec!["a", "on", "is", "ar", "en", "us", "el", "or"])
}

pub fn default_town_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Town)
        .add_field("prefix", vec!["Stone", "River", "Shadow", "Bright", "Oak", "Iron", "Mist", "Gold"])
        .add_field("suffix", vec!["dale", "ford", "keep", "haven", "port", "shire", "watch", "gate"])
}

pub fn default_dungeon_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Dungeon)
        .add_field("adjective", vec!["Ancient", "Forgotten", "Cursed", "Sunken", "Frozen", "Haunted", "Infernal", "Twisted"])
        .add_field("noun", vec!["Crypt", "Ruins", "Vault", "Labyrinth", "Sanctum", "Catacombs", "Pit", "Temple"])
}

// Add more generators as needed for items, spells, monsters, etc.
pub fn default_item_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Item)
        .add_field("adjective", vec!["Ancient", "Mystic", "Enchanted", "Cursed", "Blessed", "Forgotten", "Runed", "Royal"])
        .add_field("noun", vec!["Sword", "Amulet", "Ring", "Staff", "Crown", "Shield", "Gem", "Tome"])
}

pub fn default_spell_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::MagicSpell)
        .add_field("element", vec!["Fire", "Frost", "Shadow", "Light", "Arcane", "Storm", "Earth", "Spirit"])
        .add_field("effect", vec!["Bolt", "Blast", "Ward", "Touch", "Nova", "Curse", "Barrier", "Surge"])
}

pub fn default_monster_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Monster)
        .add_field("prefix", vec!["Dire", "Elder", "Venomous", "Shadow", "Feral", "Frost", "Infernal", "Ancient"])
        .add_field("type", vec!["Wolf", "Dragon", "Wraith", "Golem", "Serpent", "Troll", "Ogre", "Sprite"])
}

pub fn default_kingdom_name_gen() -> NameGenConfig {
    NameGenConfig::new(NameCategory::Kingdom)
        .add_field("prefix", vec!["Alder", "Storm", "Iron", "Silver", "Sun", "Frost", "Shadow", "Emerald"])
        .add_field("suffix", vec!["hold", "reach", "vale", "crest", "land", "realm", "march", "haven"])
}

// Example usage:
// let mut rng = rand::thread_rng();
// let name = default_character_name_gen().generate(&mut rng);
// println!("Generated character name: {}", name);
