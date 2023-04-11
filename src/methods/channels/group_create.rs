use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};
use crate::structures::channels::channel::Channel;

pub async fn group_create(http: &Delta, data: &DataCreateGroup) -> Result<Channel, DeltaError> {
    result(http.post(&format!("/group/create"), None).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataCreateGroup {
    /// Group name (min: 1, max: 32)
    name: String,
    /// Group description (length min: 0, max: 1024)
    description: Option<String>,
    /// Array of user IDs to add to the group
    ///
    /// Must be friends with these users.
    /// Length min: 0, max: 49
    users: Vec<String>,
    /// Whether this group is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}