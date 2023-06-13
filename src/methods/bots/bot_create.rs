use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::bot::Bot,
};

pub async fn bot_create(http: &Delta, data: &DataCreateBot) -> Result<Bot, DeltaError> {
    result(
        http.post(
            &format!("/bots/create"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

/// # Bot Details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCreateBot {
    /// Bot username
    pub name: String,
}

impl DataCreateBot {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}
