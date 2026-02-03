use reqwest::blocking::Client;
use std::error::Error;
use std::fmt;
use crate::models::{OrderBookQuery, OrderLevel, SnapshotAPIResponse, OrderbookUpdate, OrderUpdateData};

#[derive(Debug)]
pub struct OrderBook {
    pub prev_ts: u64,
    asks: Vec<OrderLevel>,
    bids: Vec<OrderLevel>
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bids = &self.bids[..5];
        let asks: &[OrderLevel] = &self.asks[..5];
        let prev_ts = self.prev_ts;

        println!("prev_ts: {}", prev_ts);
        println!("{:<10} | {:<10} | {:<10} | {:<10}\n", "Bid", "Quantity", "Ask", "Quantity");

        for (bid, ask) in bids.iter().zip(asks.iter()) {
            println!( 
                "{:<10} | {:<10} | {:<10} | {:<10}",
                bid.price, bid.quantity, ask.price, ask.quantity
            );
        }

        Ok(())
    }
}

impl OrderBook {
    pub fn update(&mut self, update: &OrderbookUpdate) {
        Self::make_update(&update.data.asks, &mut self.asks);
        Self::make_update(&update.data.bids, &mut self.bids);
        Self::truncate_bids_asks(self);
        self.prev_ts = update.ts;
    }

    fn make_update(updates: &Vec<[String; 2]>, updated_list: &mut Vec<OrderLevel>) {
        for level in updates {
            let price = level[0].clone();
            let quantity = level[1].clone();

            let mut found_index: Option<usize> = None;
            for (index, order_level) in updated_list.iter_mut().enumerate() {
                if order_level.price == price {
                    order_level.quantity = quantity.clone();
                    found_index = Some(index);
                }
            }
            
            if found_index.is_none() && quantity != "0" {
                updated_list.push(OrderLevel {
                    price: price,
                    quantity: quantity
                });
            } else if !found_index.is_none() && quantity == "0" {
                updated_list.remove(found_index.unwrap());
            }
        }
    }

    fn truncate_bids_asks(&mut self) {
        self.asks.sort_by(|a: &OrderLevel, b: &OrderLevel| b.price.cmp(&a.price));
        self.bids.sort_by(|a: &OrderLevel, b: &OrderLevel| b.price.cmp(&a.price));

        self.asks.truncate(5);
        self.bids.truncate(5);
    }
}

pub fn get_snapshot() -> Result<OrderBook, Box<dyn Error>> {
    let order_book_query: OrderBookQuery = OrderBookQuery {
        symbol: String::from("SPOT_ETH_USDT"),
        max_level: 50
    };

    println!("fetching snapshot");
    
    let mut response: SnapshotAPIResponse = Client::new()
        .get("https://api.woox.io/v3/public/orderbook")
        .query(&order_book_query)
        .send()?
        .json()?;

    println!("snapshot received");

    // println!("{response:?}");
    let mut order_book = OrderBook { 
        prev_ts: response.timestamp, 
        asks: response.data.asks, 
        bids: response.data.bids 
    };

    order_book.truncate_bids_asks();

    Ok(order_book)
}
