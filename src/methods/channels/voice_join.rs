use crate::methods::driver::{result, Delta, DeltaError};
use serde::{Deserialize, Serialize};

pub async fn message_send(http: &Delta, channel: &str) -> Result<CreateVoiceUserResponse, DeltaError> {
    result(http.post(&format!("/channels/{channel}/join_call"), None).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateVoiceUserResponse {
    /// Token for authenticating with the voice server
    token: String,
}