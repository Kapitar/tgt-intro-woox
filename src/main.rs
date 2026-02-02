mod models;
mod listener;
mod orderbook;

fn main() {
    if let Err(e) = listener::run() {
        println!("Error: {}", e);
    }
}
