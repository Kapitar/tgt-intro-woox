use tungstenite::{connect, Message};
use crate::models::{SubscribeMessage};

pub fn run() {
    let (mut ws, _resp) = connect("wss://wss.woox.io/v3/public").unwrap();

    let subscribe_message = SubscribeMessage {
        cmd: "SUBSCRIBE",
        params: ["orderbookupdate@SPOT_ETH_USDT@50"],
    };

    ws.send(Message::Text(
        serde_json::to_string(&subscribe_message).unwrap().into(),
    ))
    .unwrap();
}
