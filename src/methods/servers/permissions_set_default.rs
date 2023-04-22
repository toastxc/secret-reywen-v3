use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{permissions::exports::ServerDefaultPermissions, server::server::Server},
};

pub async fn permissions_set_default(
    http: &Delta,
    server: &str,
    data: ServerDefaultPermissions,
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
