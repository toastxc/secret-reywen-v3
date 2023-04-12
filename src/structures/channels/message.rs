use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::structures::users::user::User;
use crate::structures::server::server_member::Member;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub _id: String,
    pub nonce: String,
    pub channel: String,
    pub author: String,
    pub content: String,
    pub system: System,
    pub attachments: Vec<Attachment>,
    pub edited: String,
    pub embeds: Vec<Embed>,
    pub mentions: Vec<String>,
    pub replies: Vec<String>,
    pub reactions: Value,
    pub interactions: Interactions,
    pub masquerade: Masquerade,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Reply {
    /// Message Id
    pub id: String,
    /// Whether this reply should mention the message's author
    pub mention: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct System {
    pub r#type: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub _id: String,
    pub tag: String,
    pub filename: String,
    pub metadata: Metadata,
    pub content_type: String,
    pub size: i64,
    pub deleted: bool,
    pub reported: bool,
    pub message_id: String,
    pub user_id: String,
    pub server_id: String,
    pub object_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub r#type: String,
    pub url: String,
    pub original_url: String,
    pub special: Special,
    pub title: String,
    pub description: String,
    pub image: Image,
    pub video: Video,
    pub site_name: String,
    pub icon_url: String,
    pub colour: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SendableEmbed {
    /// Lenght min: 1, max: 128
    pub icon_url: Option<String>,
    pub url: Option<String>,
    /// Length min: 1, max: 100
    pub title: Option<String>,
    /// Length min: 1, max: 2000
    pub description: Option<String>,
    pub media: Option<String>,
    /// Length min: 1, max: 128
    pub colour: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Special {
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub width: i64,
    pub height: i64,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub url: String,
    pub width: i64,
    pub height: i64,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Interactions {
    pub reactions: Vec<String>,
    pub restrict_reactions: bool,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Masquerade {
    pub name: String,
    pub avatar: String,
    pub colour: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BulkMessageResponse {
    JustMessages(
        /// List of messages
        Vec<Message>,
    ),
    MessagesAndUsers {
        /// List of messages
        messages: Vec<Message>,
        /// List of users
        users: Vec<User>,
        /// List of members
        #[serde(skip_serializing_if = "Option::is_none")]
        members: Option<Vec<Member>>,
    },
}
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[cfg_attr(feature = "rocket_impl", derive(FromFormField))]
pub enum MessageSort {
    /// Sort by the most relevant messages
    #[default]
    Relevance,
    /// Sort by the newest messages first
    Latest,
    /// Sort by the oldest messages first
    Oldest,
}