use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{channels::channel::Channel, permissions::newcalc::PermissionData},
};

pub async fn permissions_set_default(
    http: &Delta,
    channel: &str,
    data: PermissionData,
    is_group: bool,
) -> Result<Channel, DeltaError> {
    let newdata = match is_group {
        true => serde_json::to_string(&data.value).unwrap(),
        false => serde_json::to_string(&data.field).unwrap(),
    };
    result(
        http.put(
            &format!("/channels/{channel}/permissions/default"),
            Some(&newdata),
        )
        .await,
    )
    .await
}
