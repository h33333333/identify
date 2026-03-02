use std::{net::Ipv4Addr, path::PathBuf};

use clap::{Args as ArgsDerive, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub struct Args {
    #[command(subcommand)]
    command: Command,
}

impl Args {
    /// Handles the specified command.
    pub async fn handle() -> eyre::Result<()> {
        let args = Self::parse();
        match args.command {
            Command::Run(run_command) => {
                super::commands::run::handle(run_command).await
            }
            Command::Health(health_command) => {
                super::commands::health::handle(health_command).await
            }
        }
    }
}

#[derive(Subcommand)]
enum Command {
    /// Run the API server.
    Run(RunCommand),
    /// Run a health check.
    Health(HealthCommand),
}

#[derive(ArgsDerive)]
pub struct RunCommand {
    /// Connection string for the SQLite database.
    #[arg(
        long,
        value_name = "CONNECTION_STRING",
        env = "SQLITE_CONNECTION_STRING"
    )]
    pub sqlite_connection_string: String,
    /// RSA Public Key that will be used for JWT validation.
    #[arg(long, value_name = "PATH", env = "JWT_RSA_PUBLIC_KEY_FILE")]
    pub jwt_rsa_public_key_file: PathBuf,
    /// RSA Private Key that will be used for JWT signing.
    #[arg(long, value_name = "PATH", env = "JWT_RSA_PRIVATE_KEY_FILE")]
    pub jwt_rsa_private_key_file: PathBuf,
    /// Host address.
    #[arg(
        long,
        value_name = "ADDR",
        default_value = "0.0.0.0",
        env = "HOST_ADDR"
    )]
    pub host_addr: Ipv4Addr,
    /// Port.
    #[arg(long, value_name = "PORT", default_value_t = 3000, env = "PORT")]
    pub port: u16,
}

#[derive(ArgsDerive)]
pub struct HealthCommand {
    /// Host address.
    #[arg(
        long,
        value_name = "ADDR",
        default_value = "0.0.0.0",
        env = "HOST_ADDR"
    )]
    pub host_addr: Ipv4Addr,
    /// Port.
    #[arg(long, value_name = "PORT", default_value_t = 3000, env = "PORT")]
    pub port: u16,
}
