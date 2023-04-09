use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::server::server::Server,
};

pub async fn server_fetch(http: &Delta, server: &str) -> Result<Server, DeltaError> {
    result(http.get(&format!("/servers/{server}")).await).await
}
