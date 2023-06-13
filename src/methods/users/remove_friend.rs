use crate::methods::{
    driver::{result, Delta, DeltaError},
    servers::ban_list::User,
};

pub async fn remove_friend(http: Delta, user: &str) -> Result<User, DeltaError> {
    result(http.delete(&format!("users/{user}/friend"), None).await).await
}
