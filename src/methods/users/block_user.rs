use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};
pub async fn block_user(http: Delta, user: &str) -> Result<User, DeltaError> {
    result(http.post(&format!("/users/{user}/block"), None).await).await
}
