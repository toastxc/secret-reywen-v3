use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::channel_invite::Invite;

pub async fn invite_create(http: &Delta, channel: &str) -> Result<Invite, DeltaError> {
    result(http.post(&format!("/channels/{channel}/invites"), None).await).await
}
