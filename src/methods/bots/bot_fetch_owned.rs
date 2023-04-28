use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::{bot::Bot, user::User},
};

pub async fn bot_fetch_owned(http: &Delta) -> Result<OwnedBotsResponse, DeltaError> {
    result(http.get(&format!("/bots/@me")).await).await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OwnedBotsResponse {
    pub bots: Vec<Bot>,
    pub users: Vec<User>,
}
