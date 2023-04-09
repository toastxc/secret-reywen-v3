use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::{FieldsRole, Role},
};
//https://api.revolt.chat
pub async fn roles_edit(
    http: &Delta,
    server: &str,
    role_id: &str,
    data: DataEditRole,
) -> Result<Role, DeltaError> {
    result(
        http.patch(
            &format!("/servers/{server}/roles/{role_id}"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await //https://api.revolt.chat/servers/{target}/roles/{role_id}
}

/// # Role Data
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DataEditRole {
    /// Role name
    name: Option<String>,
    /// Role colour
    colour: Option<String>,
    /// Whether this role should be displayed separately
    hoist: Option<bool>,
    /// Ranking position
    ///
    /// Smaller values take priority.
    rank: Option<i64>,
    /// Fields to remove from role object
    remove: Option<Vec<FieldsRole>>,
}

impl DataEditRole {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
