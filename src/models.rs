use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SubscribeMessage {
    pub cmd: String,
    pub params: [String; 1],
}

#[derive(Debug, Deserialize)]
pub struct OrderLevel {
    pub price: String,
    pub quantity: String
}

#[derive(Serialize)]
pub struct OrderBookQuery {
    pub symbol: String,
    pub max_level: u32
}

#[derive(Debug, Deserialize)]
pub struct SnapshotAPIResponse {
    pub success: bool,
    pub timestamp: u64,
    pub data: SnapshotAPIData
}

#[derive(Debug, Deserialize)]
pub struct SnapshotAPIData {
    pub asks: Vec<OrderLevel>,
    pub bids: Vec<OrderLevel>
}