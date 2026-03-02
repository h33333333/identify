use async_trait::async_trait;
use identify_domain::{Group, Result};
use uuid::Uuid;

/// Implementors of this contract are able to retrieve existing [Groups](identify_domain::Group) from the underlying
/// persistent storage.
#[async_trait]
pub trait Get {
    /// Get a group by its UUID.
    async fn get(&self, id: Uuid) -> Result<Group>;
}

/// Implementors of this contract are able to insert new [Groups](identify_domain::Group) into the underlying
/// persistent storage.
#[async_trait]
pub trait Insert {
    /// Insert a new group.
    async fn insert(&self, entity: &Group) -> Result<()>;
}

/// Implementors of this contract are able to associate [Users](identify_domain::User) and [Groups](identify_domain::Group) using the underlying persistent storage.
#[async_trait]
pub trait AddUser {
    /// Adds the provided user to the specified group.
    async fn add_user_to_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<()>;
}

/// Implementors of this contract are able to disassociate [Users](identify_domain::User) and [Groups](identify_domain::Group) using the underlying persistent storage.
#[async_trait]
pub trait RemoveUser {
    /// Removes the provided user from the specified group.
    async fn remove_user_from_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct ListGroupsFilters {
    /// List groups where a specific user is a member.
    pub user_id: Option<Uuid>,
}

/// Implementors of this contract are able to retrieve [Groups](identify_domain::Group) from the underlying persistent storage.
#[async_trait]
pub trait List {
    /// Retrieve groups that satisfy the provided filters.
    async fn list(&self, filters: ListGroupsFilters) -> Result<Vec<Group>>;
}
