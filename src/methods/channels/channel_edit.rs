use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::channel::{Channel, FieldsChannel};
use serde::{Deserialize, Serialize};

pub async fn channel_edit(
    http: &Delta,
    channel: &str,
    edit_data: &DataEditChannel,
) -> Result<Channel, DeltaError> {
    let data = serde_json::to_string(edit_data).unwrap();
    result(
        http.patch(&format!("/channels/{channel}"), Some(&data))
            .await,
    )
    .await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataEditChannel {
    /// Channel name
    /// length min: 1, max: 32
    pub name: Option<String>,
    /// Channel description
    /// length min: 0, max: 1024
    pub description: Option<String>,
    /// Group owner
    pub owner: Option<String>,
    /// Icon
    ///
    /// Provide an Autumn attachment Id.
    /// length min: 1, max: 128
    pub icon: Option<String>,
    /// Whether this channel is age-restricted
    pub nsfw: Option<bool>,
    /// Whether this channel is archived
    pub archived: Option<bool>,
    /// length min: 1
    pub remove: Option<Vec<FieldsChannel>>,
}

impl DataEditChannel {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self.to_owned()
    }
    pub fn set_owner(&mut self, owner: &str) -> Self {
        self.owner = Some(String::from(owner));
        self.to_owned()
    }
    pub fn set_icon(&mut self, icon: &str) -> Self {
        self.icon = Some(String::from(icon));
        self.to_owned()
    }
    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
    pub fn set_archived(&mut self, archived: bool) -> Self {
        self.archived = Some(archived);
        self.to_owned()
    }

    pub fn add_remove(&mut self, channel: FieldsChannel) -> Self {
        match self.remove.clone() {
            Some(mut channel_vec) => {
                channel_vec.push(channel);
                self.remove = Some(channel_vec);
            }
            None => self.remove = Some(vec![channel]),
        }
        self.to_owned()
    }

}
