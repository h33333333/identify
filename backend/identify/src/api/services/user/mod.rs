mod state;
use state::UserServiceState;

use axum::{Router, extract::State, response::IntoResponse, routing};

use crate::api::ApiState;

pub struct UserService;

impl UserService {
    pub fn register(router: Router<ApiState>) -> Router<ApiState> {
        router.route("/", routing::get(Self::get))
    }

    async fn get(State(state): State<UserServiceState>) -> impl IntoResponse {
        let _ = state.pool();
        "Hello world!"
    }
}
