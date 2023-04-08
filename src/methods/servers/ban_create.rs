use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct DataBanReason {
    pub reason: Option<String>,
}
pub async fn main(
    http: &Delta,
    server: &str,
    user: &str,
    reason: DataBanReason,
) -> Result<Ban, DeltaError> {
    let data = serde_json::to_string(&reason).unwrap();
    result(
        http.put(&format!("/servers/{server}/bans/{user}"), Some(&data))
            .await,
    )
    .await
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Ban {
    pub _id: BanId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BanId {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,
    pub user: Option<String>,
}
