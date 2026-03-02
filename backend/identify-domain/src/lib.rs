mod entities;

pub use entities::group::{
    Group, GroupAttrs, NewGroupAttrs,
    id::{GroupId, GroupIdAttrs},
};
pub use entities::system_settings::{
    NewSystemSettingsAttrs, SystemSettings, SystemSettingsAttrs,
};
pub use entities::user::{
    NewUserAttrs, User, UserAttrs,
    id::{UserId, UserIdAttrs},
};

use std::borrow::Cow;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DomainError>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("ID generation for {model} failed: {message}")]
    IdMismatch {
        model: Cow<'static, str>,
        message: Cow<'static, str>,
    },

    #[error("Failed to create an entity of type {entity}: {message}")]
    EntityAlreadyExists { entity: String, message: String },

    #[error("Internal error: {0}")]
    Internal(eyre::Report),

    #[error("Requested entity cannot be found")]
    NotFound,
}

impl DomainError {
    pub(crate) fn id_mismatch<
        MO: Into<Cow<'static, str>>,
        ME: Into<Cow<'static, str>>,
    >(
        model: MO,
        message: ME,
    ) -> Self {
        DomainError::IdMismatch {
            model: model.into(),
            message: message.into(),
        }
    }

    pub fn entity_already_exists<M: Into<String>>(
        entity: M,
        message: M,
    ) -> Self {
        Self::EntityAlreadyExists {
            entity: entity.into(),
            message: message.into(),
        }
    }

    pub fn internal(e: impl Into<eyre::Report>) -> Self {
        Self::Internal(e.into())
    }
}
