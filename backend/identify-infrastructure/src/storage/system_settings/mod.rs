mod row;

use async_trait::async_trait;
use eyre::eyre;
use identify_domain::{DomainError, SystemSettings};
use identify_ports::system_settings_contracts;

use crate::storage::{
    SharedTransaction, system_settings::row::SystemSettingsRow,
};

pub struct SystemSettingsRepository<'a> {
    tx: SharedTransaction<'a>,
}

impl SystemSettingsRepository<'_> {
    pub fn new<'a>(tx: SharedTransaction<'a>) -> SystemSettingsRepository<'a> {
        SystemSettingsRepository { tx }
    }
}

#[async_trait]
impl<'a> system_settings_contracts::Get for SystemSettingsRepository<'a> {
    async fn get(&self) -> Result<SystemSettings, DomainError> {
        let mut tx = self.tx.lock().await;

        let settings = sqlx::query_as!(
            SystemSettingsRow,
            r#"
                select
                    is_initialized,
                    created_at as "created_at: _",
                    updated_at as "updated_at: _"
                from
                    system_settings
                where
                    id = 1
            "#,
        )
        .fetch_one(tx.as_mut())
        .await
        .map_err(|e| DomainError::internal(eyre!(e)))
        .map(TryInto::try_into)??;

        Ok(settings)
    }
}

#[async_trait]
impl<'a> system_settings_contracts::Update for SystemSettingsRepository<'a> {
    async fn update(&self, entity: &SystemSettings) -> Result<(), DomainError> {
        let mut tx = self.tx.lock().await;

        let row: SystemSettingsRow = entity.into();

        sqlx::query!(
            r#"
                update
                    system_settings
                set
                    is_initialized = (?),
                    updated_at = (?)
                where
                    id = 1
            "#,
            row.is_initialized,
            row.updated_at
        )
        .execute(tx.as_mut())
        .await
        .map(|_| ())
        .map_err(|e| DomainError::internal(eyre!(e)))
    }
}
