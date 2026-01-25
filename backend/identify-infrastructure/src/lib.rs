use thiserror::Error;

pub mod storage;

pub type Result<T> = std::result::Result<T, InfrastructureError>;

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Internal error: {0}")]
    Internal(eyre::Report),
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
}
