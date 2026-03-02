pub(crate) mod error;
pub(crate) mod response;
pub mod user;

use async_trait::async_trait;
use axum::Router;

use crate::api::{
    ApiState,
    services::{error::ApiError, response::ApiResponse},
};

type ApiResult<T> = std::result::Result<ApiResponse<T>, ApiError>;

/// A single API service that is able to register its own endpoints on the provided [Router].
#[async_trait]
pub trait Service {
    /// Register the service's endpoints on the provided router.
    async fn get_routes(&mut self, state: ApiState) -> Router<ApiState>;
}
