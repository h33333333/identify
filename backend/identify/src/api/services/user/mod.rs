mod handlers;
mod models;
mod state;

use async_trait::async_trait;
use state::UserServiceState;

use axum::{Router, routing};

use crate::api::{
    ApiState,
    middleware::{
        authentication::{
            JwtAuthenticationLayer, NewJwtAuthenticationLayerAttrs,
        },
        authorization::PrivilegedOnlyAuthorizationLayer,
    },
    services::Service,
};

pub struct UserService;

#[async_trait]
impl Service for UserService {
    async fn get_routes(&mut self, state: ApiState) -> Router<ApiState> {
        let unprotected =
            Router::new().route("/setup", routing::post(handlers::setup));

        let requires_privileged = Router::new()
            .route("/v1/users/{user_id}", routing::get(handlers::view))
            .route_layer(PrivilegedOnlyAuthorizationLayer);

        let requires_auth = Router::new()
            .route("/v1/whoami", routing::get(handlers::whoami))
            .merge(requires_privileged)
            .layer(JwtAuthenticationLayer::new(
                NewJwtAuthenticationLayerAttrs {
                    state: state.clone(),
                },
            ));

        unprotected.merge(requires_auth)
    }
}
