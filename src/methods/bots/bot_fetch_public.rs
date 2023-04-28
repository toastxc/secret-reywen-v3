use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::users::bot::PublicBot;

pub async fn bot_fetch_public(http: &Delta, bot_id: &str) -> Result<PublicBot, DeltaError> {
    result(http.get(&format!("/bots/{bot_id}/invite")).await).await
}
