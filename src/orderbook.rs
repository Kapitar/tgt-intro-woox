use reqwest::blocking::Client;
use crate::models::{OrderBookQuery, OrderLevel, SnapshotAPIResponse};

#[derive(Debug)]
pub struct OrderBook {
    prev_ts: u64,
    asks: Vec<OrderLevel>,
    bids: Vec<OrderLevel>
}

pub fn get_snapshot() -> OrderBook {
    let order_book_query: OrderBookQuery = OrderBookQuery {
        symbol: String::from("SPOT_ETH_USDT"),
        max_level: 50
    };

    let response: SnapshotAPIResponse = Client::new()
        .get("https://api.woox.io/v3/public/orderbook")
        .query(&order_book_query)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{response:?}");

    OrderBook { 
        prev_ts: response.timestamp, 
        asks: response.data.asks, bids: 
        response.data.bids 
    }
}
