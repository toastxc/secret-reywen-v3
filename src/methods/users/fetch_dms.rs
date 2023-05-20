use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::channels::channel::Channel,
};

pub async fn fetch_dms(http: Delta) -> Result<Vec<Channel>, DeltaError> {
    result(http.get("users/friend").await).await
}
