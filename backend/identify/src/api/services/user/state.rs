use std::sync::Arc;

use identify_cache::AsyncCache;
use identify_infrastructure::auth::JwtClient;
use identify_macros::gen_model;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::api::{ApiState, CachedUserInfo};

gen_model! {
    /// A subset of state required for this service to function.
    ///
    /// It is a [substate](axum::extract::FromRef) of the [API-wide state](crate::api::InnerApiState).
    pub(super) struct UserServiceState {
        #[get(copy)]
        pool: &'static SqlitePool,
        jwt_client: Arc<JwtClient>,
        auth_cache: Arc<Box<dyn AsyncCache<Uuid, Arc<CachedUserInfo>>>>,

    }
}

impl axum::extract::FromRef<ApiState> for UserServiceState {
    fn from_ref(input: &ApiState) -> Self {
        UserServiceState {
            pool: input.pool(),
            jwt_client: input.jwt_client().clone(),
            auth_cache: input.auth_cache.clone(),
        }
    }
}
