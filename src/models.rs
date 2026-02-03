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
    pub timestamp: u64,
    pub data: SnapshotAPIData
}

#[derive(Debug, Deserialize)]
pub struct SnapshotAPIData {
    pub asks: Vec<OrderLevel>,
    pub bids: Vec<OrderLevel>
}

#[derive(Debug, Deserialize)]
pub struct OrderbookUpdate {
    pub ts: u64,
    pub data: OrderUpdateData,
}

#[derive(Debug, Deserialize)]
pub struct OrderUpdateData {
    pub asks: Vec<[String; 2]>,
    pub bids: Vec<[String; 2]>,
    #[serde(rename = "prevTs")]
    pub prev_ts: u64,
    pub ts: u64
}