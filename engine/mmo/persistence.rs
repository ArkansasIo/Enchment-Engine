//! MMO Persistence: database, save/load

#[derive(Debug, Clone, Default)]
pub struct PersistenceConfig {
    pub db_url: String,
    pub pool_size: u32,
}

pub struct CharacterRecord {/* fields for DB */}
pub struct InventoryRecord {/* fields for DB */}
pub struct EconomyRecord {/* fields for DB */}
