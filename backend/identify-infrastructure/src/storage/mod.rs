mod connection;
pub use connection::get_pool;

use std::sync::Arc;

use sqlx::SqliteTransaction;
use tokio::sync::Mutex;

pub mod users;

pub type SharedTransaction<'a> = Arc<Mutex<SqliteTransaction<'a>>>;
