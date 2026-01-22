mod use_cases;

pub use use_cases::{
    CreateUserParams, UserUseCaseDeps, create_user, user_contracts,
};

use thiserror::Error;

pub type Result<T> = std::result::Result<T, ApplicationError>;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] identify_domain::DomainError),

    #[error("Internal error: {0}")]
    Internal(eyre::Report),

    #[error(
        "Failed to create an entity of type {entity} because it already exists: {message}"
    )]
    EntityAlreadyExists { entity: String, message: String },
}

impl ApplicationError {
    pub fn internal(e: impl Into<eyre::Report>) -> Self {
        Self::Internal(e.into())
    }

    pub fn internal_with_message<M: Into<String>>(
        e: impl Into<eyre::Report>,
        message: M,
    ) -> Self {
        Self::Internal(e.into().wrap_err(message.into()))
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
}
