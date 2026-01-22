use axum::{Router, routing::get};
use eyre::{Context, Result};
use identify::logging;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    logging::init().wrap_err("error while initializing the logging")?;

    info!("Initializing");

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
