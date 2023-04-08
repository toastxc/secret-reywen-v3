use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

pub async fn main(
    http: &Delta,
    server: &str,
    member: &str,
    data: DataMemberEdit,
) -> Result<DataMemberEdit, DeltaError> {
    result(
        http.patch(
            &format!("/servers/{server}/members/{member}"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataMemberEdit {
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub roles: Vec<String>,
    pub timeout: Option<String>,
    pub remove: Option<Vec<String>>,
}

// https://api.revolt.chat/servers/{server}/members/{target}
