use crate::methods::driver::{result, Delta, DeltaError};

pub async fn message_delete(http: &Delta, channel: &str, message: &str) -> Result<(), DeltaError> {
    result(
        http.delete(&format!("/channels/{channel}/messages/{message}"), None)
            .await,
    )
    .await
}
