//! MMO Backend: account, auth, analytics

#[derive(Debug, Clone, Default)]
pub struct Account {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Default)]
pub struct Session {
    pub account_id: u64,
    pub token: String,
    pub expires_at: u64,
}
