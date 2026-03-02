use async_trait::async_trait;
use identify_domain::{Result, User};
use uuid::Uuid;

use crate::pagination::PaginatedResponse;

/// Implementors of this contract are able to retrieve existing [Users](identify_domain::User) from the underlying
/// persistent storage.
#[async_trait]
pub trait Get {
    /// Get a user by their UUID.
    async fn get(&self, id: Uuid) -> Result<User>;
}

/// Implementors of this contract are able to insert new [Users](identify_domain::User) into the underlying
/// persistent storage.
#[async_trait]
pub trait Insert {
    /// Insert a new user.
    async fn insert(&self, entity: &User) -> Result<()>;
}

/// User with additional details.
#[derive(Debug, Clone)]
pub struct ExtendedUser {
    pub user: User,
    /// Whether at least one of the user's groups is privileged.
    pub is_privileged: bool,
}

#[derive(Debug, Clone)]
pub struct ListUsersFilters {}

/// Implementors of this contract are able to retrieve [Users](identify_domain::User) from the underlying persistent storage.
#[async_trait]
pub trait List {
    /// Retrieve users that satisfy the provided filters.
    async fn list(
        &self,
        filters: ListUsersFilters,
    ) -> Result<PaginatedResponse<ExtendedUser>>;
}
