use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{permissions::exports::ServerPermissions, server::server::Server},
};

pub async fn permissions_set(
    http: &Delta,
    server: &str,
    role_id: &str,
    data: ServerPermissions,
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
