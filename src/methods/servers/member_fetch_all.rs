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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ResponseMemberAll {
    /// List of members
    pub members: Vec<Member>,
    /// List of users
    pub users: Vec<User>,
}

impl ResponseMemberAll {
    pub fn new() -> Self {
        Default::default()
    }

    /// Removes all offline users to reduce lag on front end
    pub fn rm_offline(&self) -> Self {
        let mut newuser = Self::new();

        for member in &self.members {
            for user in &self.users {
                if user.id == member.id.user && user.online == Some(true) {
                    newuser.members.push(member.clone());
                    newuser.users.push(user.clone());
                }
            }
        }
        newuser
    }

    // automatically applies rm_offline if member count exceeds specified limit
    pub fn lagmax(&mut self, max: usize) -> Self {
        if self.users.len() >= max {
            Self::rm_offline(&self)
        } else {
            self.to_owned()
        }
    }
}
