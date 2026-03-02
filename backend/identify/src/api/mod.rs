use std::sync::Arc;

use identify_cache::AsyncCache;
use identify_domain::{Group, User};
use identify_infrastructure::auth::JwtClient;
use identify_macros::gen_model;
use sqlx::SqlitePool;
use uuid::Uuid;

pub mod middleware;
pub mod services;

/// Alias to simplify signatures.
pub type ApiState = Arc<InnerApiState>;

gen_model! {
    /// API-wide state.
    ///
    /// Each service is expected to extract only the parts it needs.
    pub struct InnerApiState {
        #[get(copy)]
        pool: &'static SqlitePool,
        #[new(type(JwtClient))]
        jwt_client: Arc<JwtClient>,
        /// Used for caching user details to avoid doing a DB query on every request.
        #[get(clone)]
        auth_cache: Arc<Box<dyn AsyncCache<Uuid, Arc<CachedUserInfo>>>>,
    }

    pub struct NewInnerApiStateAttrs;
}

impl InnerApiState {
    pub fn new(attrs: NewInnerApiStateAttrs) -> Self {
        InnerApiState {
            pool: attrs.pool,
            jwt_client: Arc::new(attrs.jwt_client),
            auth_cache: attrs.auth_cache,
        }
    }
}

/// Cached user-related information.
#[derive(Debug, Clone)]
pub struct CachedUserInfo {
    user: User,
    /// Whether at least one of the groups contains in [Self::groups] is privileged.
    is_privileged: bool,
    groups: Vec<Group>,
}
