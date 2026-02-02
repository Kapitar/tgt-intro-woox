use tungstenite::{connect, Message};
use crate::orderbook;
use crate::{models::SubscribeMessage, orderbook::OrderBook};

pub fn run() {
    // let (mut ws, _resp) = connect("wss://wss.woox.io/v3/public").unwrap();

    // let subscribe_message: SubscribeMessage = SubscribeMessage {
    //     cmd: String::from("SUBSCRIBE"),
    //     params: [String::from("orderbookupdate@SPOT_ETH_USDT@50")],
    // };

    // ws.send(Message::Text(
    //     serde_json::to_string(&subscribe_message).unwrap().into(),
    // )).unwrap();

    let order_book: OrderBook = orderbook::get_snapshot();
    order_book.print();
}