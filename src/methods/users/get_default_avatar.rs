
use crate::methods::driver::{result, Delta, DeltaError};

pub async fn get_default_avatar(http: Delta, user: &str) -> Result<String, DeltaError> {
    result(http.get(&format!("users/{user}/default_avatar")).await).await
}
