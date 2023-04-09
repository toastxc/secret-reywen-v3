use crate::methods::driver::{result, Delta, DeltaError};

pub async fn server_delete(http: &Delta, server: &str) -> Result<(), DeltaError> {
    result(http.delete(&format!("/servers/{server}"), None).await).await
}
