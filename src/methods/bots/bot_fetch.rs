use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::{bot::Bot, user::User},
};

pub async fn bot_fetch(http: &Delta, bot_id: &str) -> Result<BotResponse, DeltaError> {
    result(http.get(&format!("/bots/{bot_id}")).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BotResponse {
    pub bot: Bot,
    pub user: User,
}
