use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{channels::channel::Channel, server::server::Server},
};

pub async fn server_create(
    http: &Delta,
    data: DataCreateServer,
) -> Result<CreateServerResponse, DeltaError> {
    result(
        http.post(
            &format!("/servers/create"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataCreateServer {
    /// Server name
    name: String,
    /// Server description
    description: Option<String>,
    /// Whether this server is age-restricted
    #[serde(skip_serializing_if = "Option::is_none")]
    nsfw: Option<bool>,
}

impl DataCreateServer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
}

/// # Create Server Response
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateServerResponse {
    /// Server object
    server: Server,
    /// Default channels
    channels: Vec<Channel>,
}
