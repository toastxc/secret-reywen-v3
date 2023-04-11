use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

pub async fn message_bulk_delete(http: &Delta, channel: &str, messages: &DataBulkDelete) -> Result<(), DeltaError> {
    let data = serde_json::to_string(messages).unwrap();
    result(http.delete(&format!("/channels/{channel}/messages/bulk"), Some(&data)).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataBulkDelete {
    /// Message IDs
    ids: Vec<String>,
}