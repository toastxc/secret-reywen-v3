use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::channels::channel::Channel,
};

pub async fn open_dm(http: Delta, user: &str) -> Result<Channel, DeltaError> {
    result(http.get(&format!("users/{user}/dm")).await).await
}
