use serde::{Deserialize, Serialize};

use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::{server::server_member::Member, users::user::User},
};

pub async fn main(http: &Delta, server: &str) -> Result<AllMemberResponse, DeltaError> {
    result(http.get(&format!("/servers/{server}/members")).await).await
}

/// # Member List
///
/// Both lists are sorted by ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllMemberResponse {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}
