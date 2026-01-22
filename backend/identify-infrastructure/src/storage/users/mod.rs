mod row;

use async_trait::async_trait;
use eyre::eyre;
use identify_application::{ApplicationError, user_contracts};
use identify_domain::User;
use uuid::Uuid;

use crate::storage::{SharedTransaction, users::row::UserRow};

pub struct UsersRepository<'a> {
    tx: SharedTransaction<'a>,
}

impl UsersRepository<'_> {
    pub fn new<'a>(tx: SharedTransaction<'a>) -> UsersRepository<'a> {
        UsersRepository { tx }
    }
}

#[async_trait]
impl<'a> user_contracts::Get for UsersRepository<'a> {
    async fn get(&self, id: Uuid) -> Result<User, ApplicationError> {
        let mut tx = self.tx.lock().await;

        let user = sqlx::query_as!(
            UserRow,
            r#"
                select
                    id as "id: Uuid",
                    email,
                    first_name,
                    last_name,
                    created_at as "created_at: _",
                    updated_at as "updated_at: _"
                from
                    users
                where
                    id = (?)
            "#,
            id
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e| ApplicationError::internal(eyre!(e)))
        .map(TryInto::try_into)??;

        Ok(user)
    }
}

#[async_trait]
impl<'a> user_contracts::Insert for UsersRepository<'a> {
    async fn insert(&self, entity: &User) -> Result<(), ApplicationError> {
        let mut tx = self.tx.lock().await;

        let row: UserRow = entity.into();

        sqlx::query!(
            r#"
                insert into users (
                    id,
                    email,
                    first_name,
                    last_name,
                    created_at,
                    updated_at
                ) values (
                    (?),
                    (?),
                    (?),
                    (?),
                    (?),
                    (?)
                )
            "#,
            row.id,
            row.email,
            row.first_name,
            row.last_name,
            row.created_at,
            row.updated_at
        )
        .execute(tx.as_mut())
        .await
        .map(|_| ())
        .map_err(|e| match e.as_database_error() {
            Some(db_error) if db_error.is_unique_violation() => {
                ApplicationError::entity_already_exists(
                    "User",
                    "Email is already taken",
                )
            }
            _ => ApplicationError::internal(eyre!(e)),
        })
    }
}
