use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{channels::channel::Channel, permissions::newcalc::PermissionData},
};

pub async fn permissions_set(
    http: &Delta,
    channel: &str,
    role_id: &str,
    data: PermissionData,
) -> Result<Channel, DeltaError> {
    result(
        http.put(
            &format!("/channels/{channel}/permissions/{role_id}"),
            Some(&serde_json::to_string(&data.field).unwrap()),
        )
        .await,
    )
    .await
}
