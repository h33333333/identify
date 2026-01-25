use axum::{Router, routing::get};
use eyre::{Context, Result};
use identify::logging;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init().wrap_err("error while initializing the logging")?;

    info!("Initializing!");

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
