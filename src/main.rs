use reywen_revolt::{client::federolt, websocket::data::Websocket};
#[tokio::main]
async fn main() {
    federolt().await;
}
