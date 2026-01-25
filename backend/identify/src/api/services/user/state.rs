use identify_macros::gen_model;
use sqlx::SqlitePool;

use crate::api::ApiState;

gen_model! {
    /// A subset of state required for this service to function.
    ///
    /// It is a [substate](axum::extract::FromRef) of [API-wide state](crate::api::InnerApiState).
    pub(super) struct UserServiceState {
        #[get(copy)]
        pool: &'static SqlitePool,
    }
}

impl axum::extract::FromRef<ApiState> for UserServiceState {
    fn from_ref(input: &ApiState) -> Self {
        UserServiceState { pool: input.pool() }
    }
}
