use crate::methods::driver::{result, Delta, DeltaError};
//https://api.revolt.chat
pub async fn roles_delete(http: &Delta, server: &str, role_id: &str) -> Result<(), DeltaError> {
    result(
        http.delete(&format!("/servers/{server}/roles/{role_id}"), None)
            .await,
    )
    .await
}
