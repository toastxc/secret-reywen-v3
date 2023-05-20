use serde::{Deserialize, Serialize};

use crate::methods::{
    driver::{result, Delta, DeltaError},
    servers::ban_list::User,
};

pub async fn sed_friend_request(
    http: Delta,
    data: DataSendFriendRequest,
) -> Result<User, DeltaError> {
    result(
        http.post("users/friend", Some(&serde_json::to_string(&data).unwrap()))
            .await,
    )
    .await
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataSendFriendRequest {
    pub username: String,
}
impl DataSendFriendRequest {
    pub fn set_username(username: &str) -> Self {
        Self {
            username: String::from(username),
        }
    }
}
