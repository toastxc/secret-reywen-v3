use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};
use crate::structures::channels::message::{SendableEmbed, Message};

pub async fn message_edit(http: &Delta, channel: &str, message: &str, edit_data: &DataEditMessage) -> Result<Message, DeltaError> {
    let data = serde_json::to_string(edit_data).unwrap();
    result(http.patch(&format!("/channels/{channel}/messages/{message}"), Some(&data)).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditMessage {
    /// New message content (length min: 1, length max: 2000)
    content: Option<String>,
    /// Embeds to include in the message (length min: 0, length max: 10)
    embeds: Option<Vec<SendableEmbed>>,
}