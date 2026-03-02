mod row;

use async_trait::async_trait;
use eyre::eyre;
use identify_domain::{DomainError, Group};
use identify_ports::group_contracts::{self, ListGroupsFilters};
use uuid::Uuid;

use crate::storage::SharedTransaction;
use row::GroupRow;

pub struct GroupsRepository<'a> {
    tx: SharedTransaction<'a>,
}

impl GroupsRepository<'_> {
    pub fn new<'a>(tx: SharedTransaction<'a>) -> GroupsRepository<'a> {
        GroupsRepository { tx }
    }
}

#[async_trait]
impl<'a> group_contracts::Get for GroupsRepository<'a> {
    async fn get(&self, id: Uuid) -> Result<Group, DomainError> {
        let mut tx = self.tx.lock().await;

        let group = sqlx::query_as!(
            GroupRow,
            r#"
                select
                    id as "id: Uuid",
                    name,
                    is_privileged,
                    created_at as "created_at: _",
                    updated_at as "updated_at: _"
                from
                    groups
                where
                    id = (?)
            "#,
            id
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e| {
            if matches!(e, sqlx::Error::RowNotFound) {
                return DomainError::NotFound;
            }
            DomainError::internal(eyre!(e))
        })
        .map(TryInto::try_into)??;

        Ok(group)
    }
}

#[async_trait]
impl<'a> group_contracts::Insert for GroupsRepository<'a> {
    async fn insert(&self, entity: &Group) -> Result<(), DomainError> {
        let mut tx = self.tx.lock().await;

        let row: GroupRow = entity.into();

        sqlx::query!(
            r#"
                insert into groups (
                    id,
                    name,
                    is_privileged,
                    created_at,
                    updated_at
                ) values (
                    (?),
                    (?),
                    (?),
                    (?),
                    (?)
                )
            "#,
            row.id,
            row.name,
            row.is_privileged,
            row.created_at,
            row.updated_at
        )
        .execute(tx.as_mut())
        .await
        .map(|_| ())
        .map_err(|e| match e.as_database_error() {
            Some(db_error) if db_error.is_unique_violation() => {
                DomainError::entity_already_exists(
                    "Group",
                    "Group name is already in use",
                )
            }
            _ => DomainError::internal(eyre!(e)),
        })
    }
}

#[async_trait]
impl<'a> group_contracts::AddUser for GroupsRepository<'a> {
    async fn add_user_to_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<(), DomainError> {
        let mut tx = self.tx.lock().await;

        sqlx::query!(
            r#"
                insert into users_groups (
                    user_id,
                    group_id
                ) values (
                    (?),
                    (?)
                )
            "#,
            user_id,
            group_id
        )
        .execute(tx.as_mut())
        .await
        .map(|_| ())
        .map_err(|e| match e.as_database_error() {
            Some(db_error) if db_error.is_unique_violation() => {
                DomainError::entity_already_exists(
                    "UserGroup",
                    "User already belongs to the specified group",
                )
            }
            _ => DomainError::internal(eyre!(e)),
        })
    }
}

#[async_trait]
impl<'a> group_contracts::RemoveUser for GroupsRepository<'a> {
    async fn remove_user_from_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
    ) -> Result<(), DomainError> {
        let mut tx = self.tx.lock().await;

        sqlx::query!(
            r#"
                delete from users_groups
                where
                        user_id = (?)
                    and
                        group_id = (?)
            "#,
            user_id,
            group_id
        )
        .execute(tx.as_mut())
        .await
        .map(|_| ())
        .map_err(DomainError::internal)
    }
}

#[async_trait]
impl<'a> group_contracts::List for GroupsRepository<'a> {
    async fn list(
        &self,
        filters: ListGroupsFilters,
    ) -> Result<Vec<Group>, DomainError> {
        let mut tx = self.tx.lock().await;

        let groups = sqlx::query_as!(
            GroupRow,
            r#"
                select
                    id as "id: Uuid",
                    name,
                    is_privileged,
                    created_at as "created_at: _",
                    updated_at as "updated_at: _"
                from
                    groups
                where
                    $1 is null or id in (
                        select
                            group_id
                        from
                            users_groups
                        where
                            user_id = $1
                    )
            "#,
            filters.user_id,
        )
        .fetch_all(tx.as_mut())
        .await
        .map_err(DomainError::internal)?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>, DomainError>>()?;

        Ok(groups)
    }
}
