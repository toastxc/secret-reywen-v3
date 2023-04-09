use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::{Category, FieldsServer, Server, SystemMessageChannels},
};

pub async fn server_edit(
    http: &Delta,
    server: &str,
    data: DataEditServer,
) -> Result<Server, DeltaError> {
    result(
        http.patch(
            &format!("/servers/{server}"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

/// # Server Data
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DataEditServer {
    /// Server name
    pub name: Option<String>,
    /// Server description
    pub description: Option<String>,
    /// Attachment Id for icon
    pub icon: Option<String>,
    /// Attachment Id for banner
    pub banner: Option<String>,
    /// Category structure for server
    pub categories: Option<Vec<Category>>,
    /// System message configuration
    pub system_messages: Option<SystemMessageChannels>,

    /// Bitfield of server flags
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,

    // Whether this server is age-restricted
    // nsfw: Option<bool>,
    /// Whether this server is public and should show up on [Revolt Discover](https://rvlt.gg)
    discoverable: Option<bool>,
    /// Whether analytics should be collected for this server
    ///
    /// Must be enabled in order to show up on [Revolt Discover](https://rvlt.gg).
    analytics: Option<bool>,

    /// Fields to remove from server object
    remove: Option<Vec<FieldsServer>>,
}

impl DataEditServer {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
