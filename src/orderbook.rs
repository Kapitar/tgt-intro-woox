use reqwest::blocking::Client;
use crate::models::{OrderBookQuery, OrderLevel, SnapshotAPIResponse};

#[derive(Debug)]
pub struct OrderBook {
    prev_ts: u64,
    asks: Vec<OrderLevel>,
    bids: Vec<OrderLevel>
}

impl OrderBook {
    pub fn print(self) {
        let bids = &self.bids[..5];
        let asks = &self.asks[..5];

        println!("Bid | Quantity | Ask | Quantity");
        
        for (bid, ask) in bids.iter().zip(asks.iter()) {
            println!(
                "{} | {} | {} | {}",
                bid.price, bid.quantity, ask.price, ask.quantity
            );
        }
    }
}

pub fn get_snapshot() -> OrderBook {
    let order_book_query: OrderBookQuery = OrderBookQuery {
        symbol: String::from("SPOT_ETH_USDT"),
        max_level: 50
    };

    let mut response: SnapshotAPIResponse = Client::new()
        .get("https://api.woox.io/v3/public/orderbook")
        .query(&order_book_query)
        .send()
        .unwrap()
        .json()
        .unwrap();

    // println!("{response:?}");

    response.data.asks.sort_by(|a: &OrderLevel, b: &OrderLevel| b.price.cmp(&a.price));

    OrderBook { 
        prev_ts: response.timestamp, 
        asks: response.data.asks, 
        bids: response.data.bids 
    }
}
