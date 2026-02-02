use serde::Serialize;

#[derive(Serialize)]
pub struct SubscribeMessage<'a> {
    pub cmd: &'a str,
    pub params: [&'a str; 1],
}