use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::UserProfile,
};

pub async fn fetch_profile(http: Delta, user: &str) -> Result<Vec<UserProfile>, DeltaError> {
    result(http.get(&format!("/users/{user}/profile")).await).await
}
