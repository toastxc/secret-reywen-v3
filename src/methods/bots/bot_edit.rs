use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::bot::{Bot, FieldsBot},
};

pub async fn bot_edit(http: &Delta, bot_id: &str, data: &DataEditBot) -> Result<Bot, DeltaError> {
    result(
        http.patch(
            &format!("/bots/{bot_id}"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

/// # Bot Details
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataEditBot {
    /// Bot username
    pub name: Option<String>,
    /// Whether the bot can be added by anyone
    pub public: Option<bool>,
    /// Whether analytics should be gathered for this bot
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    pub analytics: Option<bool>,
    /// Interactions URL
    pub interactions_url: Option<String>,
    /// Fields to remove from bot object
    pub remove: Option<Vec<FieldsBot>>,
}

impl DataEditBot {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn set_name(&mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self.to_owned()
    }
    pub fn set_public(&mut self, public: bool) -> Self {
        self.public = Some(public);
        self.to_owned()
    }
    pub fn set_analytics(&mut self, analytics: bool) -> Self {
        self.analytics = Some(analytics);
        self.to_owned()
    }
    pub fn set_interactions_url(&mut self, interactions_url: &str) -> Self {
        self.interactions_url = Some(String::from(interactions_url));
        self.to_owned()
    }

    pub fn add_remove(&mut self, field: FieldsBot) -> Self {
        match self.remove.clone() {
            Some(mut old) => old.push(field),
            None => self.remove = Some(vec![field]),
        }
        self.to_owned()
    }
    pub fn set_remove(&mut self, fields: Vec<FieldsBot>) -> Self {
        self.remove = Some(fields);
        self.to_owned()
    }
}
