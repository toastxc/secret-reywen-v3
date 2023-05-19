use crate::methods::driver::{result, Delta, DeltaError};
use crate::structures::channels::message::{BulkMessageResponse2, MessageSort};
use serde::{Deserialize, Serialize};

pub async fn message_search(
    http: &Delta,
    channel: &str,
    search_options: &DataMessageSearch,
) -> Result<BulkMessageResponse2, DeltaError> {
    let data = serde_json::to_string(search_options).unwrap();
    result(
        http.post(&format!("/channels/{channel}/search"), Some(&data))
            .await,
    )
    .await
}

/// # Search Parameters
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DataMessageSearch {
    /// Full-text search query
    ///
    /// See [MongoDB documentation](https://docs.mongodb.com/manual/text-search/#-text-operator) for more information.
    /// length min: 1, max: 64
    pub query: String,

    /// Maximum number of messages to fetch
    /// length min: 1, max: 100
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    pub after: Option<String>,
    /// Message sort direction
    ///
    /// By default, it will be sorted by latest.
    pub sort: MessageSort,
    /// Whether to include user (and member, if server channel) objects
    pub include_users: Option<bool>,
}

impl DataMessageSearch {
    pub fn new(query: &str) -> Self {
        Self {
            query: String::from(query),
            ..Default::default()
        }
    }
    pub fn set_limit(&mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self.to_owned()
    }
    pub fn set_before(&mut self, before: &str) -> Self {
        self.before = Some(String::from(before));
        self.to_owned()
    }
    pub fn set_after(&mut self, after: &str) -> Self {
        self.after = Some(String::from(after));
        self.to_owned()
    }
    pub fn set_sort(&mut self, sort: MessageSort) -> Self {
        self.sort = sort;
        self.to_owned()
    }
    pub fn set_include_users(&mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self.to_owned()
    }
}
