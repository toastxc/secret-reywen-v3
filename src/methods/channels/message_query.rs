use crate::methods::corelib::struct_to_url;
use crate::methods::driver::{result, Delta, DeltaError};

use crate::structures::channels::message::{BulkMessageResponse2, MessageSort};
use serde::{Deserialize, Serialize};

pub async fn message_query(
    http: &Delta,
    channel: &str,
    query: &DataQueryMessages,
) -> Result<BulkMessageResponse2, DeltaError> {
    result(
        http.get(&format!(
            "/channels/{channel}/messages{}",
            struct_to_url(query)
        ))
        .await,
    )
    .await
}

/// # Query Parameters
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataQueryMessages {
    /// Maximum number of messages to fetch
    ///
    /// For fetching nearby messages, this is \`(limit + 1)\`.
    /// min: 1, max 100
    pub limit: Option<i64>,
    /// Message id before which messages should be fetched
    /// length min: 26, max: 26
    pub before: Option<String>,
    /// Message id after which messages should be fetched
    /// length min: 26, max: 26
    pub after: Option<String>,
    /// Message sort direction
    pub sort: Option<MessageSort>,
    /// Message id to search around
    ///
    /// Specifying 'nearby' ignores 'before', 'after' and 'sort'.
    /// It will also take half of limit rounded as the limits to each side.
    /// It also fetches the message ID specified.
    /// length min: 26, max: 26
    pub nearby: Option<String>,
    /// Whether to include user (and member, if server channel) objects
    pub include_users: Option<bool>,
}

impl DataQueryMessages {
    pub fn new() -> Self {
        Self {
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
        self.sort = Some(sort);
        self.to_owned()
    }
    pub fn set_nearby(&mut self, nearby: &str) -> Self {
        self.nearby = Some(String::from(nearby));
        self.to_owned()
    }

    pub fn set_include_users(&mut self, include_users: bool) -> Self {
        self.include_users = Some(include_users);
        self.to_owned()
    }
}
