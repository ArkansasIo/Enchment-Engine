//! MMO Networking: server/client, sync, events

#[derive(Debug, Clone, Default)]
pub struct NetworkConfig {
    pub tick_rate: u32,
    pub max_players: u32,
    pub server_addr: String,
}

pub enum NetworkEvent {
    PlayerConnect { id: u64 },
    PlayerDisconnect { id: u64 },
    PositionUpdate { id: u64, x: f32, y: f32, z: f32 },
    CombatAction { id: u64, action: String },
    ChatMessage { id: u64, message: String },
}
