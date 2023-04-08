use crate::methods::driver::{result, Delta, DeltaError};

pub async fn ban_remove(http: &Delta, server: &str, user: &str) -> Result<(), DeltaError> {
    result(
        http.delete(&format!("/servers/{server}/bans/{user}"), None)
            .await,
    )
    .await
}
