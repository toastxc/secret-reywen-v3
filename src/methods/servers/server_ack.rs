use crate::methods::driver::{result, Delta, DeltaError};

pub async fn server_ack(http: &Delta, server: &str) -> Result<(), DeltaError> {
    result(http.put(&format!("/servers/{server}/ack"), None).await).await
}
