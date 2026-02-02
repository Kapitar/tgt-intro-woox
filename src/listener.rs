use tungstenite::{connect, Message};
use std::error::Error;

use crate::orderbook;
use crate::{models::SubscribeMessage, orderbook::OrderBook};

pub fn run() -> Result<(), Box<dyn Error>> {
    println!("trying to connect to WS");
    let (mut ws, _resp) = connect("wss://wss.woox.io/v3/public")?;
    println!("connected to WS");

    let subscribe_message: SubscribeMessage = SubscribeMessage {
        cmd: String::from("SUBSCRIBE"),
        params: [String::from("orderbookupdate@SPOT_ETH_USDT@50")],
    };

    ws.send(Message::Text(
        serde_json::to_string(&subscribe_message)?.into(),
    ))?;

    let order_book: OrderBook = orderbook::get_snapshot()?;
    order_book.print();

    loop {
        let order_update = ws.read()?.into_text()?;
        println!("{order_update}");
    }
}