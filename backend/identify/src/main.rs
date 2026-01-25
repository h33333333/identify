use std::sync::Arc;

use axum::Router;
use eyre::{Context, Result};
use identify::{
    api::{InnerApiState, services::user::UserService},
    logging,
};
use identify_infrastructure::storage::get_pool;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init().wrap_err("error while initializing the logging")?;

    info!("Initializing");

    let router = Router::new();
    let router = UserService::register(router);

    let pool = get_pool("sqlite://data.db").await?;

    let router = router.with_state(Arc::new(InnerApiState::new(
        identify::api::NewInnerApiStateAttrs { pool },
    )));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    Ok(())
}
