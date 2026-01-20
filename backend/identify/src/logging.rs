use eyre::eyre;
use identify_core::{Error, Result};
use tracing_subscriber::EnvFilter;

pub const LOGGING_ENV: &str = "IDENTIFY_LOG";
pub const LOGGING_FILE_ENV: &str = "IDENTIFY_FILE_LOG";

pub fn init() -> Result<()> {
    let env_filter = EnvFilter::builder()
        .with_env_var(LOGGING_ENV)
        .try_from_env()
        .unwrap_or_else(|_| "identify=info".into());

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_env_filter(env_filter)
        .try_init()
        .map_err(|e| {
            Error::internal_with_message(
                eyre!(e),
                "error while initializing the logging",
            )
        })
}
