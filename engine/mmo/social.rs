//! MMO Social: guilds, parties, chat

#[derive(Debug, Clone, Default)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub members: Vec<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct Party {
    pub id: String,
    pub members: Vec<u64>,
}

#[derive(Debug, Clone, Default)]
pub struct FriendList {
    pub player_id: u64,
    pub friends: Vec<u64>,
}
