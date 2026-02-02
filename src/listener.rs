use tungstenite::{connect, Message};
use crate::orderbook;
use crate::{models::SubscribeMessage, orderbook::OrderBook};

pub fn run() {
    let order_book = orderbook::get_snapshot();
    println!("{order_book:?}")
    
    // let (mut ws, _resp) = connect("wss://wss.woox.io/v3/public").unwrap();

    // let subscribe_message = SubscribeMessage {
    //     cmd: "SUBSCRIBE",
    //     params: ["orderbookupdate@SPOT_ETH_USDT@50"],
    // };

    // ws.send(Message::Text(
    //     serde_json::to_string(&subscribe_message).unwrap().into(),
    // ))
    // .unwrap();
}