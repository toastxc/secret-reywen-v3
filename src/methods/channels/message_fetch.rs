use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::message::Message;

pub async fn message_fetch(http: &Delta, channel: &str, message: &str) -> Result<Message, DeltaError> {
    result(http.get(&format!("/channels/{channel}/messages/{message}")).await).await
}