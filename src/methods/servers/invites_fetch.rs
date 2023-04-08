use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

pub async fn main(http: &Delta, server: &str) -> Result<Vec<DataInvite>, DeltaError> {
    result(http.get(&format!("/servers/{server}/invites")).await).await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataInvite {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "_id")]
    pub id: String,
    pub server: String,
    pub creator: String,
    pub channel: String,
}
