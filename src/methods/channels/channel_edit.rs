use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};
use crate::structures::channels::channel::{FieldsChannel, Channel};

pub async fn channel_edit(http: &Delta, channel: &str, edit_data: &DataEditChannel) -> Result<Channel, DeltaError> {
    let data = serde_json::to_string(edit_data).unwrap();
    result(http.patch(&format!("/channels/{channel}"), Some(&data)).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditChannel {
    /// Channel name
    /// length min: 1, max: 32
    name: Option<String>,
    /// Channel description
    /// length min: 0, max: 1024
    description: Option<String>,
    /// Group owner
    owner: Option<String>,
    /// Icon
    ///
    /// Provide an Autumn attachment Id.
    /// length min: 1, max: 128
    icon: Option<String>,
    /// Whether this channel is age-restricted
    nsfw: Option<bool>,
    /// Whether this channel is archived
    archived: Option<bool>,
    /// length min: 1
    remove: Option<Vec<FieldsChannel>>,
}