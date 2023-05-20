use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};

pub async fn fetch_user(http: Delta, user: &str) -> Result<User, DeltaError> {
    result(http.get(&format!("users/{user}")).await).await
}
