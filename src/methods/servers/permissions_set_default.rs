use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{permissions::newcalc::PermissionData, server::server::Server},
};

pub async fn permissions_set_default(
    http: &Delta,
    server: &str,
    data: PermissionData,
) -> Result<Server, DeltaError> {
    result(
        http.put(
            &format!("/servers/{server}/permissions/default"),
            Some(&serde_json::to_string(&data.value).unwrap()),
        )
        .await,
    )
    .await
}
