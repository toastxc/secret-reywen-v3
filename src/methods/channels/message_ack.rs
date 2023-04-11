use crate::methods::driver::{result, Delta, DeltaError};

pub async fn message_ack(http: &Delta, channel: &str, message: &str) -> Result<(), DeltaError> {
    result(http.put(&format!("/channels/{channel}/ack/{message}"), None).await).await
}
