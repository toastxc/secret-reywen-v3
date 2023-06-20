use serde::Deserialize;

use crate::methods::driver::{result, Delta, DeltaError};

pub async fn find_mutual(http: Delta, user: &str) -> Result<MutualResponse, DeltaError> {
    result(http.get(&format!("users/{user}/mutual")).await).await
}

/// # Mutual Friends and Servers Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct MutualResponse {
    /// Array of mutual user IDs that both users are friends with
    pub users: Vec<String>,
    /// Array of mutual server IDs that both users are in
    pub servers: Vec<String>,
}
