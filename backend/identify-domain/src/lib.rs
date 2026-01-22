mod entities;

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
}

impl DomainError {
    pub fn id_mismatch<
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
}
