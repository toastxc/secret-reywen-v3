use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
pub async fn unblock_user(http: Delta, user: &str) -> Result<User, DeltaError> {
    result(http.delete(&format!("/users/{user}/block"), None).await).await
}
