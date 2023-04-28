use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::users::user::User;

pub async fn members_fetch(http: &Delta, group_channel: &str) -> Result<Vec<User>, DeltaError> {
    result(
        http.put(&format!("/channels/{group_channel}/members"), None)
            .await,
    )
    .await
}
