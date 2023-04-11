use crate::methods::driver::{result, Delta, DeltaError};

pub async fn group_remove_member(http: &Delta, channel: &str, member: &str) -> Result<(), DeltaError> {
    result(http.delete(&format!("/channels/{channel}/recipients/{member}"), None).await).await
}
