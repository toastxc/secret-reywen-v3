use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::Server,
};

pub async fn permissions_set_default(
    http: &Delta,
    server: &str,
    data: DataPermissionSet,
) -> Result<Server, DeltaError> {
    result(
        http.put(
            &format!("/servers/{server}/permissions/default"),
            Some(&serde_json::to_string(&data).unwrap()),
        )
        .await,
    )
    .await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataPermissionSet {
    pub permissions: u64,
}

impl DataPermissionSet {
    pub fn new(permissions: u64) -> Self {
        Self { permissions }
    }
}
