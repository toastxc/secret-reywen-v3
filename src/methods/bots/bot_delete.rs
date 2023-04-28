use crate::methods::driver::{result, Delta, DeltaError};

pub async fn bot_delete(http: &Delta, bot_id: &str) -> Result<(), DeltaError> {
    result(http.delete(&format!("/bots/{bot_id}"), None).await).await
}
