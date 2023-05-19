use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{server::server_member::Member, users::user::User},
};

pub async fn member_fetch_all(http: &Delta, server: &str) -> Result<ResponseMemberAll, DeltaError> {
    result(http.get(&format!("/servers/{server}/members")).await).await
}

/// # Member List
///
/// Both lists are sorted by ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseMemberAll {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}
