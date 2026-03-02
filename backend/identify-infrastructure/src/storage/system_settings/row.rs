use chrono::{DateTime, Utc};
use identify_domain::{DomainError, SystemSettings, SystemSettingsAttrs};

pub struct SystemSettingsRow {
    pub is_initialized: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&SystemSettings> for SystemSettingsRow {
    fn from(value: &SystemSettings) -> Self {
        let attrs = value.to_attributes();

        SystemSettingsRow {
            is_initialized: attrs.is_initialized,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        }
    }
}

impl TryFrom<SystemSettingsRow> for SystemSettings {
    type Error = DomainError;

    fn try_from(value: SystemSettingsRow) -> Result<Self, Self::Error> {
        SystemSettings::load(SystemSettingsAttrs {
            is_initialized: value.is_initialized,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
