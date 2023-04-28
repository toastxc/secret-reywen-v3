use serde::{Deserialize, Serialize};

use crate::methods::driver::{result, Delta, DeltaError};

pub async fn bot_invite(http: &Delta, bot_id: &str, server_id: &str) -> Result<(), DeltaError> {
    let data = DataBotInvite {
        server: server_id.to_string(),
    };
    result(
        http.post(
            &format!("/bots/{bot_id}/invite"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataBotInvite {
    pub server: String,
}
