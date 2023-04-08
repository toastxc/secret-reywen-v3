use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct DataChangeUsername {
    pub username: String,
    pub password: String,
}
pub async fn change_username(http: Delta, data: DataChangeUsername) -> Result<User, DeltaError> {
    let data = serde_json::to_string(&data).unwrap();

    result(http.post("users/friend", Some(&data)).await).await
}
