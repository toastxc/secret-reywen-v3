use crate::{
    methods::driver::{result, Delta, DeltaError},
    structures::users::user::User,
};

pub async fn fetch_self(http: Delta) -> Result<User, DeltaError> {
    result(http.get("users/@me").await).await
}
