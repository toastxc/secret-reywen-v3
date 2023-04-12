use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::message::{BulkMessageResponse, MessageSort};
use serde::{Deserialize, Serialize};

pub async fn message_search(http: &Delta, channel: &str, search_options: &OptionsMessageSearch) -> Result<BulkMessageResponse, DeltaError> {
    let data = serde_json::to_string(search_options).unwrap();
    result(http.post(&format!("/channels/{channel}/search"), Some(&data)).await).await
}

/// # Search Parameters
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OptionsMessageSearch {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    /// length min: 1, max: 64
    query: String,

    /// Maximum number of messages to fetch
    /// length min: 1, max: 100
    limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by latest.
    sort: MessageSort,
    /// Whether to include user (and member, if server channel) objects
    include_users: Option<bool>,
}