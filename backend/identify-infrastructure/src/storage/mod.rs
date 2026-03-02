mod connection;
pub use connection::get_pool;

pub mod groups;
pub mod system_settings;
pub mod users;

use std::sync::Arc;

use sqlx::SqliteTransaction;
use tokio::sync::{Mutex, MutexGuard};

use crate::InfrastructureError;

#[derive(Debug, Clone)]
pub struct SharedTransaction<'a>(Arc<Mutex<SqliteTransaction<'a>>>);

impl<'a> SharedTransaction<'a> {
    pub fn new(tx: SqliteTransaction<'a>) -> Self {
        SharedTransaction(Arc::new(Mutex::new(tx)))
    }

    pub async fn lock(&self) -> MutexGuard<'_, SqliteTransaction<'a>> {
        self.0.lock().await
    }

    pub fn into_inner(
        self,
    ) -> Result<SqliteTransaction<'a>, InfrastructureError> {
        self.try_into()
    }
}

impl<'a> From<SqliteTransaction<'a>> for SharedTransaction<'a> {
    fn from(value: SqliteTransaction<'a>) -> Self {
        SharedTransaction::new(value)
    }
}

impl<'a> TryFrom<SharedTransaction<'a>> for SqliteTransaction<'a> {
    type Error = InfrastructureError;

    fn try_from(value: SharedTransaction<'a>) -> Result<Self, Self::Error> {
        Arc::try_unwrap(value.0)
            .map(|mutex| mutex.into_inner())
            .map_err(|_| InfrastructureError::ManyReferencesToTransaction)
    }
}

pub trait ToShared<'a>
where
    Self: 'a,
{
    fn to_shared(self) -> SharedTransaction<'a>;
}

impl<'a> ToShared<'a> for SqliteTransaction<'a> {
    fn to_shared(self) -> SharedTransaction<'a> {
        self.into()
    }
}
