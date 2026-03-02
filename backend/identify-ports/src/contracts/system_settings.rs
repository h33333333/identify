use async_trait::async_trait;
use identify_domain::{Result, SystemSettings};

/// Implementors of this contract are able to retrieve [System settings](identify_domain::SystemSettings) from the underlying
/// persistent storage.
#[async_trait]
pub trait Get {
    /// Get system settings.
    async fn get(&self) -> Result<SystemSettings>;
}

/// Implementors of this contract are able to update [System settings](identify_domain::SystemSettings) in the underlying persistent storage.
#[async_trait]
pub trait Update {
    /// Update system settings.
    async fn update(&self, entity: &SystemSettings) -> Result<()>;
}
