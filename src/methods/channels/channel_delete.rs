use crate::methods::driver::{result, Delta, DeltaError};

pub async fn channel_delete(http: &Delta, channel: &str) -> Result<(), DeltaError> {
    result(http.delete(&format!("/channels/{channel}"), None).await).await
}
