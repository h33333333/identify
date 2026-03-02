use crate::{AsyncCache, CacheError};

use super::Result;
use std::{hash::Hash, num::NonZeroU64, pin::Pin};

use async_trait::async_trait;
use moka::future::Cache;

#[async_trait]
impl<K, V> super::AsyncCache<K, V> for Cache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
{
    /// Returns the cached entry by its key if it's present in the cache.
    async fn get(&self, key: &K) -> Result<Option<V>> {
        Ok(Cache::get(&self, key).await)
    }

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
    ) -> Result<V> {
        Cache::try_get_with(self, key, fetcher)
            .await
            .map_err(|e| CacheError::internal(eyre::eyre!(e)))
    }

    /// Inserts a new entry under the specified key.
    async fn insert(&self, key: K, value: V) -> Result<()> {
        Cache::insert(&self, key, value).await;
        Ok(())
    }

    /// Deletes an entry stored under the specified key.
    ///
    /// Returns the deleted entry if it was present and [None](Option::None) otherwise.
    async fn delete(&self, key: &K) -> Result<Option<V>> {
        Ok(Cache::remove(&self, key).await)
    }

    /// Evicts all cached entries.
    async fn clear(&self) -> Result<()> {
        Ok(Cache::invalidate_all(&self))
    }
}

/// Constructs a new LFU cache of the provided size.
pub fn new_lfu_cache<
    K: Hash + Eq + Send + Sync + 'static,
    V: Send + Sync + Clone + 'static,
>(
    size: NonZeroU64,
) -> Result<Box<dyn AsyncCache<K, V>>> {
    Ok(Box::new(Cache::new(size.into())))
}
