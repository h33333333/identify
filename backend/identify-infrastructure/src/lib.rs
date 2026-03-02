use thiserror::Error;

pub mod auth;
pub mod password;
pub mod storage;

pub type Result<T> = std::result::Result<T, InfrastructureError>;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Internal error: {0}")]
    Internal(eyre::Report),

    #[error("Error while using one of the clients: {0}")]
    ClientError(eyre::Report),

    #[error("Error while validating a JWT token: {0}")]
    JwtValidationError(#[from] jsonwebtoken::errors::Error),

    #[error("Failed to unwrap the shared transaction: multiple pointers exist")]
    ManyReferencesToTransaction,
}

impl InfrastructureError {
    pub fn internal(e: impl Into<eyre::Report>) -> Self {
        Self::Internal(e.into())
    }

    pub fn internal_with_message<M: Into<String>>(
        e: impl Into<eyre::Report>,
        message: M,
    ) -> Self {
        Self::Internal(e.into().wrap_err(message.into()))
    }

    pub fn client_with_message<M: Into<String>>(
        e: impl Into<eyre::Report>,
        message: M,
    ) -> Self {
        Self::ClientError(e.into().wrap_err(message.into()))
    }
}
