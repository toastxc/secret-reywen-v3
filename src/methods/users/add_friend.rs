use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
use serde::{Deserialize, Serialize};

pub async fn add_friend(http: Delta, username: &str) -> Result<User, DeltaError> {
    let data = serde_json::to_string(&(DataFriendRequest::from_str(username))).unwrap();

    result(http.post("users/friend", Some(&data)).await).await
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct DataFriendRequest {
    username: String,
}

impl DataFriendRequest {
    pub fn from_str(username: &str) -> Self {
        Self {
            username: String::from(username),
        }
    }
}
