use eyre::{Context, Result};
use identify::{cli, logging};

#[tokio::main]
async fn main() -> Result<()> {
    logging::init().wrap_err("Error while initializing the logging")?;
    cli::run().await
}
