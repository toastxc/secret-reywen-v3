use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct DataFriendRequest {
    username: String,
}
pub async fn add_friend(http: Delta, username: String) -> Result<User, DeltaError> {
    let data = serde_json::to_string(&(DataFriendRequest { username })).unwrap();

    result(http.post("users/friend", Some(&data)).await).await
}
