mod models;

pub use models::user::{
    NewUserAttrs, User, UserAttrs,
    id::{UserId, UserIdAttrs},
};

use std::borrow::Cow;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(eyre::Report),

    #[error("ID generation for {model} failed: {message}")]
    IdMismatch {
        model: Cow<'static, str>,
        message: Cow<'static, str>,
    },
}

impl Error {
    pub fn internal(e: impl Into<eyre::Report>) -> Self {
        Self::Internal(e.into())
    }

    pub fn internal_with_message<M: Into<String>>(
        e: impl Into<eyre::Report>,
        message: M,
    ) -> Self {
        Self::Internal(e.into().wrap_err(message.into()))
    }

    pub fn id_mismatch<
        MO: Into<Cow<'static, str>>,
        ME: Into<Cow<'static, str>>,
    >(
        model: MO,
        message: ME,
    ) -> Self {
        Error::IdMismatch {
            model: model.into(),
            message: message.into(),
        }
    }
}
