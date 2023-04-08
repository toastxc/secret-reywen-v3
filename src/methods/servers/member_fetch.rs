use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server_member::Member,
};

pub async fn member_fetch(http: &Delta, server: &str, member: &str) -> Result<Member, DeltaError> {
    result(
        http.get(&format!("/servers/{server}/members/{member}"))
            .await,
    )
    .await
}
