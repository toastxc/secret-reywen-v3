use crate::methods::driver::Delta;
pub const SERVER: &str = "01GKWVWGHN242DVWG4BKXG2C7F";
pub const USER: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
pub const ROLE: &str = "01GXFR9FPEPFY188X5MKV2E8ZN";
pub const CHANNEL: &str = "01GKWVWGHNBNCFPC9Q7CRDHBVZ";
pub const GROUP: &str = "01GYM0JBNKWRJYX56F9GYABS4R";
pub const BOT: &str = "01GXF9E5H7K6BSJ6Q9QGWYRVWD";
// enter values here for testing

pub async fn tester_bot() -> Delta {
    Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header("x-bot-token", include_str!("bot-token.txt"))
        .unwrap()
        .set_timeout(10)
}

pub async fn tester_user() -> Delta {
    Delta::new()
        .set_url("https://api.revolt.chat/")
        .add_header("x-session-token", include_str!("self-token.txt"))
        .unwrap()
        .set_timeout(10)
}
