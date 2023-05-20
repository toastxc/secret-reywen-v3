use serde::Deserialize;

use crate::methods::driver::{result, Delta, DeltaError};

pub async fn fetch_user(http: Delta, user: &str) -> Result<ResponseFlag, DeltaError> {
    result(http.get(&format!("users/{user}")).await).await
}
/// # Flag Response
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ResponseFlag {
    /// Flags
    pub flags: i32,
}
