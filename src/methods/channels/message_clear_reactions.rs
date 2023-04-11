use crate::methods::driver::{result, Delta, DeltaError};

pub async fn message_clear_reactions(http: &Delta, channel: &str, message: &str) -> Result<(), DeltaError> {
    result(http.delete(&format!("/channels/{channel}/messages/{message}/reactions"), None).await).await
}
