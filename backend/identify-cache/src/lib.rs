#[cfg(feature = "moka")]
mod lfu;
use std::pin::Pin;

#[cfg(feature = "moka")]
pub use lfu::new_lfu_cache;

use async_trait::async_trait;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, CacheError>;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Internal error: {0}")]
    InternalError(eyre::Report),
}

impl CacheError {
    pub(crate) fn internal<M: Into<eyre::Report>>(message: M) -> Self {
        CacheError::InternalError(eyre::eyre!(message))
    }
}

/// Implementors of this trait are able to cache and retrieve entries.
#[async_trait]
pub trait AsyncCache<K, V>: Send + Sync {
    /// Returns the cached entry by its key if it's present in the cache.
    async fn get(&self, key: &K) -> Result<Option<V>>;

    /// Returns the cached entry if it's present or executes the provided fetcher populate it
    /// first.
    async fn try_get_or_insert(
        &self,
        key: K,
        fetcher: Pin<
            Box<
                dyn Future<Output = std::result::Result<V, eyre::Report>>
                    + Send,
            >,
        >,
    ) -> Result<V>;

    /// Inserts a new entry under the specified key.
    async fn insert(&self, key: K, value: V) -> Result<()>;

    /// Deletes an entry stored under the specified key.
    ///
    /// Returns the deleted entry if it was present and [None](Option::None) otherwise.
    async fn delete(&self, key: &K) -> Result<Option<V>>;

    /// Evicts all cached entries.
    async fn clear(&self) -> Result<()>;
}
