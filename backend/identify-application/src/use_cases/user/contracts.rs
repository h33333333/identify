use crate::Result;
use async_trait::async_trait;
use identify_domain::User;
use uuid::Uuid;

/// Implementors of this contract are able retrieve existing [Users](crate::User) from the underlying
/// persistent storage.
#[async_trait]
pub trait Get {
    /// Get a user by their UUID.
    async fn get(&self, id: Uuid) -> Result<User>;
}

/// Implementors of this contract are able to insert new [Users](crate::User) into the underlying
/// persistent storage.
#[async_trait]
pub trait Insert {
    /// Insert a new user.
    async fn insert(&self, entity: &User) -> Result<()>;
}
