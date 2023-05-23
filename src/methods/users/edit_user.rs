use serde::Serialize;

use crate::{
    methods::{
        driver::{result, Delta, DeltaError},
        servers::ban_list::User,
    },
    structures::users::user::{FieldsUser, UserStatus},
};

pub async fn edit_user(http: Delta, user: &str, data: DataEditUser) -> Result<User, DeltaError> {
    let data = serde_json::to_string(&data).unwrap();

    result(http.patch(&format!("users/{user}"), Some(&data)).await).await
}

//https://api.revolt.chat/users/{target}
/// # User Data
#[derive(Serialize, Debug, Clone, Default)]
pub struct DataEditUser {
    /// Attachment Id for avatar
    pub avatar: Option<String>,
    /// New user status
    pub status: Option<UserStatus>,
    /// New user profile data
    ///
    /// This is applied as a partial.
    pub profile: Option<UserProfileData>,
    /// Bitfield of user badges
    pub badges: Option<i32>,
    /// Enum of user flags
    pub flags: Option<i32>,
    /// Fields to remove from user object
    pub remove: Option<Vec<FieldsUser>>,
}

impl DataEditUser {
    pub fn set_avatar(&mut self, avatar: &str) -> Self {
        self.avatar = Some(String::from(avatar));
        self.to_owned()
    }
    pub fn set_status(&mut self, status: UserStatus) -> Self {
        self.status = Some(status);
        self.to_owned()
    }
    pub fn set_profile(&mut self, profile: UserProfileData) -> Self {
        self.profile = Some(profile);
        self.to_owned()
    }
    pub fn set_badges(&mut self, badges: i32) -> Self {
        self.badges = Some(badges);
        self.to_owned()
    }
    pub fn set_flags(&mut self, flags: i32) -> Self {
        self.flags = Some(flags);
        self.to_owned()
    }
    pub fn set_remove(&mut self, remove: Vec<FieldsUser>) -> Self {
        self.remove = Some(remove);
        self.to_owned()
    }
    pub fn add_remove(&mut self, remove: FieldsUser) -> Self {
        match self.remove.clone() {
            Some(mut data) => {
                data.push(remove);
                self.remove = Some(data);
            }
            None => self.remove = Some(vec![remove]),
        }
        self.to_owned()
    }

    pub fn new() -> Self {
        Default::default()
    }
}
#[derive(Serialize, Debug, Clone, Default)]
pub struct UserProfileData {
    /// Text to set as user profile description
    content: Option<String>,
    /// Attachment Id for background
    background: Option<String>,
}

impl UserProfileData {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn set_content(&mut self, content: &str) -> Self {
        self.content = Some(String::from(content));
        self.to_owned()
    }
    pub fn set_background(&mut self, background: &str) -> Self {
        self.background = Some(String::from(background));
        self.to_owned()
    }
}
