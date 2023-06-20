use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{media::attachment::File, server::server_ban::ServerBan, users::user::User},
};

pub async fn ban_list(http: &Delta, server: &str) -> Result<DataBanList, DeltaError> {
    result(http.get(&format!("/servers/{server}/bans")).await).await
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BannedUser {
    /// Id of the banned user
    #[serde(rename = "_id")]
    pub id: String,
    /// Username of the banned user
    pub username: String,
    /// Avatar of the banned user
    pub avatar: Option<File>,
}

/// # Ban List Result
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DataBanList {
    /// Users objects
    users: Vec<BannedUser>,
    /// Ban objects
    bans: Vec<ServerBan>,
}

impl From<User> for BannedUser {
    fn from(user: User) -> Self {
        BannedUser {
            id: user.id,
            username: user.username,
            avatar: user.avatar,
        }
    }
}
