pub(crate) mod args;
pub(crate) mod commands;
pub(crate) mod util;

pub async fn run() -> eyre::Result<()> {
    args::Args::handle().await
}
