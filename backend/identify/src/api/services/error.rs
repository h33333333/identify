use std::{borrow::Cow, fmt::Display};

use axum::{Json, http::StatusCode, response::IntoResponse};
use identify_domain::DomainError;
use identify_infrastructure::InfrastructureError;
use serde::Serialize;
use tracing::error;

#[derive(Debug)]
pub struct ApiError {
    status_code: StatusCode,
    message: Cow<'static, str>,
    error: Option<eyre::Report>,
}

impl ApiError {
    pub fn new(
        message: impl Into<Cow<'static, str>>,
        status_code: StatusCode,
        error: Option<eyre::Report>,
    ) -> Self {
        ApiError {
            status_code,
            message: message.into(),
            error,
        }
    }

    pub fn internal(error: Option<impl Into<eyre::Report>>) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Failed to process the request: internal error".into(),
            error: error.map(Into::into),
        }
    }

    pub fn unauthorized(error: Option<impl Into<eyre::Report>>) -> Self {
        ApiError {
            status_code: StatusCode::UNAUTHORIZED,
            message: "Authorization validation failed".into(),
            error: error.map(Into::into),
        }
    }

    pub fn bad_request(
        error: Option<impl Into<eyre::Report>>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: message.into(),
            error: error.map(Into::into),
        }
    }

    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        ApiError {
            status_code: StatusCode::NOT_FOUND,
            message: message.into(),
            error: None,
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "API error (code={}): {}", self.status_code, self.message)
    }
}

impl std::error::Error for ApiError {}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        if self.status_code >= StatusCode::INTERNAL_SERVER_ERROR {
            error!(
                http_code = ?self.status_code,
                message = ?self.message,
                error = ?self.error,
                "Internal error, might need to investigate"
            );
        }

        (
            self.status_code,
            Json(ErrorResponse {
                // We don't expose the internal error here, only the user-friendly message.
                error: self.message,
            }),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        // All errors that we encounter in use cases are converted to ApplicationError, so these
        // errors we have here are internal ones and happened outside of use cases.
        ApiError::internal(Some(value))
    }
}

impl From<InfrastructureError> for ApiError {
    fn from(value: InfrastructureError) -> Self {
        match value {
            err @ (InfrastructureError::Internal(_)
            | InfrastructureError::ClientError(_)) => Self::internal(Some(err)),
            InfrastructureError::JwtValidationError(err) => {
                Self::unauthorized(Some(err))
            }
            InfrastructureError::ManyReferencesToTransaction => {
                Self::internal(Some(eyre::eyre!(
                    "Bug: holding multiple references to a SharedTransaction at the end of a handler"
                )))
            }
        }
    }
}

impl From<DomainError> for ApiError {
    fn from(value: DomainError) -> Self {
        match value {
            err @ (DomainError::Internal(_)
            | DomainError::IdMismatch { .. }) => Self::internal(Some(err)),
            DomainError::EntityAlreadyExists { entity, message } => {
                Self::bad_request(Some(eyre::eyre!(entity)), message)
            }
            DomainError::NotFound => Self::not_found(value.to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: Cow<'static, str>,
}
