use crate::methods::driver::{result, Delta, DeltaError};

pub async fn group_add_member(http: &Delta, group: &str, member: &str) -> Result<(), DeltaError> {
    result(
        http.put(&format!("/channels/{group}/recipients/{member}"), None)
            .await,
    )
    .await
}
