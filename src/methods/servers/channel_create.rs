use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::channels::channel::Channel,
};
use serde::{Deserialize, Serialize};

pub async fn main(
    http: &Delta,
    server: &str,
    data: DataChannelCreate,
) -> Result<Channel, DeltaError> {
    result(
        http.post(
            &format!("/servers/{server}/channels"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataChannelCreate {
    #[serde(rename = "type")]
    pub r#type: Option<ChannelType>,
    pub name: String,
    pub description: Option<String>,
    pub nsfw: Option<bool>,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum ChannelType {
    #[default]
    Text,
    Voice,
}

impl DataChannelCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            r#type: Some(ChannelType::Text),
            ..Default::default()
        }
    }
}
