pub mod bots;
pub mod channels;
pub mod common;
pub mod servers;
pub mod users;
pub mod websocket;

use reywen_http::driver::Delta;

use crate::client::Client;

pub const SERVER: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
pub const USER: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
pub const CHANNEL: &str = "01GKWVWGHNBNCFPC9Q7CRDHBVZ";
pub const GROUP: &str = "01GYM0JBNKWRJYX56F9GYABS4R";
pub const BOT: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
// enter values here for testing

fn tester_bot() -> Delta {
    Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header("x-bot-token", include_str!("bot-token.txt"))
        .unwrap()
        .set_timeout(10)
}

fn tester_user() -> Delta {
    Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header("x-session-token", include_str!("self-token.txt"))
        .unwrap()
        .set_timeout(10)
}

pub async fn test_client(is_bot: bool) -> Client {
    let mut client = Client::new();

    client.http = match is_bot {
        true => tester_bot(),
        false => tester_user(),
    };

    client
}
