use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::message::{BulkMessageResponse, MessageSort};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub async fn message_fetch(http: &Delta, channel: &str, message: &str, query: &OptionsQueryMessages) -> Result<BulkMessageResponse, DeltaError> {
    let data = serde_json::to_string(query).unwrap();
    let data_map: HashMap<String, String> = serde_json::from_str(&data).unwrap();
    let mut url: String = format!("/channels/{channel}/messages").to_string();
    let params_vec: Vec<String> = data_map.into_iter().map(|(k, v)| format!("{k}={v}")).collect();
    if params_vec.len() > 0 {
        url.push_str("?");
        url.push_str(params_vec.join("&").as_str());
    }
    println!("{:#?}", url);
    result(http.get(url.as_str()).await).await
}

/// # Query Parameters
#[derive(Serialize, Deserialize)]
pub struct OptionsQueryMessages {
    /// Maximum number of messages to fetch
    ///
    /// For fetching nearby messages, this is \`(limit + 1)\`.
    /// min: 1, max 100
    limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    after: Option<String>,
    /// Message sort direction
    sort: Option<MessageSort>,
    /// Message id to search around
    ///
    /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
    /// It will also take half of limit rounded as the limits to each side.
    /// It also fetches the message ID specified.
    /// length min: 26, max: 26
    nearby: Option<String>,
    /// Whether to include user (and member, if server channel) objects
    include_users: Option<bool>,
}