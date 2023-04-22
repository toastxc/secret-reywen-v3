use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::channel::Channel;
use serde::{Deserialize, Serialize};

pub async fn group_create(http: &Delta, data: &DataCreateGroup) -> Result<Channel, DeltaError> {
    result(http.post(&format!("channels/create"), None).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataCreateGroup {
    /// Group name (min: 1, max: 32)
    pub name: String,
    /// Group description (length min: 0, max: 1024)
    pub description: Option<String>,
    /// Array of user IDs to add to the group
    ///
    /// Must be friends with these users.
    /// Length min: 0, max: 49
    pub users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,
}

impl DataCreateGroup {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
    pub fn set_description(&mut self, description: &str) -> Self {
        self.description = Some(String::from(description));
        self.to_owned()
    }

    pub fn add_user(&mut self, user: &str) -> Self {
        self.users.push(String::from(user));
        self.to_owned()
    }
    pub fn set_users(&mut self, users: Vec<&str>) -> Self {
        let mut user_vec = Vec::new();
        for x in users {
            user_vec.push(String::from(x));
        }
        self.users = user_vec;
        self.to_owned()
    }

    pub fn set_nsfw(&mut self, nsfw: bool) -> Self {
        self.nsfw = Some(nsfw);
        self.to_owned()
    }
}
