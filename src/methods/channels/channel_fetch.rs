use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::channel::Channel;

pub async fn channel_fetch(http: &Delta, channel: &str) -> Result<Channel, DeltaError> {
    result(http.get(&format!("/channels/{channel}")).await).await
}
