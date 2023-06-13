use serde::{Deserialize, Serialize};

use crate::methods::{
    corelib::struct_to_url,
    driver::{result, Delta, DeltaError},
};

pub async fn message_unreact(
    http: &Delta,
    channel: &str,
    message: &str,
    emoji: &str,
    data: &DataUnreact,
) -> Result<(), DeltaError> {
    result(
        http.delete(
            &format!(
                "/channels/{channel}/messages/{message}/reactions/{emoji}{}",
                struct_to_url(data)
            ),
            None,
        )
        .await,
    )
    .await
}

/// # Query Parameters
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataUnreact {
    /// Remove a specific user's reaction
    pub user_id: Option<String>,
    /// Remove all reactions
    pub remove_all: Option<bool>,
}

impl DataUnreact {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_user_id(&mut self, user_id: &str) -> Self {
        self.user_id = Some(String::from(user_id));
        self.to_owned()
    }
    pub fn set_remove_all(&mut self, remove_all: bool) -> Self {
        self.remove_all = Some(remove_all);
        self.to_owned()
    }
}
