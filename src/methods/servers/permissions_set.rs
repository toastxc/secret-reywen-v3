use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{permissions::newcalc::PermissionData, server::server::Server},
};

pub async fn permissions_set(
    http: &Delta,
    server: &str,
    role_id: &str,
    data: PermissionData,
) -> Result<Server, DeltaError> {
    result(
        http.put(
            &format!("/servers/{server}/permissions/{role_id}"),
            Some(&serde_json::to_string(&data.field).unwrap()),
        )
        .await,
    )
    .await
}
