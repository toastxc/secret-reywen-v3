use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::Role,
};
//https://api.revolt.chat
pub async fn roles_create(
    http: &Delta,
    server: &str,
    data: DataRoleCreate,
) -> Result<NewRoleResponse, DeltaError> {
    result(
        http.post(
            &format!("/servers/{server}/roles"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataRoleCreate {
    pub name: String,
    pub rank: Option<u32>,
}

impl DataRoleCreate {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            ..Default::default()
        }
    }
}

/// # New Role Response
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewRoleResponse {
    /// Id of the role
    id: String,
    /// New role
    role: Role,
}
