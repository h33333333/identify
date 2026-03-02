use std::{
    net::SocketAddrV4, num::NonZeroU64, path::Path, sync::Arc, time::Duration,
};

use eyre::Context as _;
use identify_infrastructure::{auth::JwtClient, storage::get_pool};
use tracing::info;

use crate::{
    api::{
        InnerApiState, NewInnerApiStateAttrs,
        services::{Service as _, user::UserService},
    },
    cli::{args, util::cancellation_signal},
};

pub async fn handle(cmd: args::RunCommand) -> eyre::Result<()> {
    let addr = SocketAddrV4::new(cmd.host_addr, cmd.port);

    info!("Starting the API server on {}", addr);

    let pool = get_pool(&cmd.sqlite_connection_string).await?;

    let (private_key, public_key) = read_rsa_keys(
        &cmd.jwt_rsa_private_key_file,
        &cmd.jwt_rsa_public_key_file,
    )
    .await?;

    let jwt_client =
        JwtClient::new(&private_key, &public_key, Duration::from_secs(3600))
            .wrap_err("Failed to initialize a JWT client")?;

    let auth_cache =
        identify_cache::new_lfu_cache::<_, _>(NonZeroU64::new(100).unwrap())
            .wrap_err("Failed to initialize auth cache")?;

    let state = Arc::new(InnerApiState::new(NewInnerApiStateAttrs {
        pool,
        jwt_client,
        auth_cache: Arc::new(auth_cache),
    }));

    let router = UserService.get_routes(state.clone()).await;
    let router = router.with_state::<()>(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .wrap_err("Failed to bind a TCP listener")?;

    axum::serve(listener, router)
        .with_graceful_shutdown(cancellation_signal())
        .await
        .wrap_err("Error while serving the requests")
}

/// Reads private and public key and returns the raw data.
async fn read_rsa_keys(
    private_key: &Path,
    public_key: &Path,
) -> eyre::Result<(Vec<u8>, Vec<u8>)> {
    let private_key = tokio::fs::read(private_key)
        .await
        .wrap_err("Failed to read RSA private key")?;
    let public_key = tokio::fs::read(public_key)
        .await
        .wrap_err("Failed to read RSA public key")?;
    Ok((private_key, public_key))
}
