use crate::methods::driver::{result, Delta, DeltaError};

pub async fn message_react(
    http: &Delta,
    channel: &str,
    message: &str,
    emoji: &str,
) -> Result<(), DeltaError> {
    result(
        http.put(
            &format!("/channels/{channel}/messages/{message}/reactions/{emoji}"),
            None,
        )
        .await,
    )
    .await
}
