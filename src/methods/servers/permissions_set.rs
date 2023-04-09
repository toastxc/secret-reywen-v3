use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::Server,
};

pub async fn permissions_set(
    http: &Delta,
    server: &str,
    role_id: &str,
    data: DataPermissionSet,
) -> Result<Server, DeltaError> {
    result(
        http.put(
            &format!("/servers/{server}/permissions/{role_id}"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissionSet {
    pub permissions: DataPermissions,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissions {
    pub allow: u32,
    pub deny: u32,
}
