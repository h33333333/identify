use std::time::Duration;

use crate::{InfrastructureError, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use tokio::sync::OnceCell;

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn get_pool<C: AsRef<str>>(
    connection_string: C,
) -> Result<&'static SqlitePool> {
    // TODO: provide all options in a config struct
    POOL.get_or_try_init(|| async {
        SqlitePoolOptions::default()
            .min_connections(1)
            .max_connections(10)
            .max_lifetime(Duration::from_secs(30 * 60))
            .idle_timeout(Duration::from_secs(3 * 60))
            .connect_lazy(connection_string.as_ref())
    })
    .await
    .map_err(|e| {
        InfrastructureError::internal_with_message(
            e,
            "Failed to initialize a DB pool",
        )
    })
}
