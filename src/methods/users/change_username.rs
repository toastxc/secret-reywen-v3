use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
use serde::Serialize;

pub async fn change_username(http: Delta, data: DataChangeUsername) -> Result<User, DeltaError> {
    let data = serde_json::to_string(&data).unwrap();

    result(http.post("users/friend", Some(&data)).await).await
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct DataChangeUsername {
    pub username: String,
    pub password: String,
}

impl DataChangeUsername {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_username(&mut self, username: &str) -> Self {
        self.username = String::from(username);
        self.to_owned()
    }
    pub fn set_password(&mut self, password: &str) -> Self {
        self.password = String::from(password);
        self.to_owned()
    }
}
