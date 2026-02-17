//! MMO Economy: trading, auction, currency

#[derive(Debug, Clone, Default)]
pub struct AuctionListing {
    pub id: String,
    pub seller: u64,
    pub item_id: String,
    pub price: u64,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Default)]
pub struct Trade {
    pub id: String,
    pub from: u64,
    pub to: u64,
    pub item_ids: Vec<String>,
    pub currency: u64,
}

#[derive(Debug, Clone, Default)]
pub struct Currency {
    pub player_id: u64,
    pub amount: u64,
}
