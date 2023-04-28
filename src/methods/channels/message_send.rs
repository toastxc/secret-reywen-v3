use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::message::{
    Interactions, Masquerade, Message, Reply, SendableEmbed,
};
use serde::{Deserialize, Serialize};

pub async fn message_send(
    http: &Delta,
    channel: &str,
    message: &DataMessageSend,
) -> Result<Message, DeltaError> {
    let data = serde_json::to_string(message).unwrap();
    result(
        http.post(&format!("/channels/{channel}/messages"), Some(&data))
            .await,
    )
    .await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataMessageSend {
    /// Message content to send
    /// length min: 0, max: 2000
    pub content: Option<String>,
    /// Attachments to include in message
    /// length min: 1, max: 128
    pub attachments: Option<Vec<String>>,
    /// Messages to reply to
    pub replies: Option<Vec<Reply>>,
    /// Embeds to include in message
    ///
    /// Text embed content contributes to the content length cap
    /// length min: 1, max: 10
    pub embeds: Option<Vec<SendableEmbed>>,
    /// Masquerade to apply to this message
    pub masquerade: Option<Masquerade>,
    /// Information about how this message should be interacted with
    pub interactions: Option<Interactions>,
}

impl DataMessageSend {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_content(&mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.to_owned()
    }

    pub fn set_masquerade(&mut self, masquerade: &Masquerade) -> Self {
        self.masquerade = Some(masquerade.clone());
        self.to_owned()
    }
}
