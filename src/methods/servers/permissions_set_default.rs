use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::Server,
};

pub async fn permissions_set_default(
    http: &Delta,
    server: &str,
    data: DataPermissionSetDefault,
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
pub struct DataPermissionSetDefault {
    pub permissions: u64,
}

impl DataPermissionSetDefault {
    pub fn new(permissions: u64) -> Self {
        Self { permissions }
    }
}
