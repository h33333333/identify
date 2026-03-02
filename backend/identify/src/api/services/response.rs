use std::borrow::Cow;

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum ApiResponse<T> {
    #[allow(unused)]
    Empty,
    #[allow(unused)]
    Message(Cow<'static, str>),
    Data(T),
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        let message = match self {
            ApiResponse::Empty => ApiMessage::default(),
            ApiResponse::Message(message) => ApiMessage {
                data: None,
                message: Some(message),
            },
            ApiResponse::Data(data) => ApiMessage {
                data: Some(data),
                message: None,
            },
        };

        (StatusCode::OK, Json(message)).into_response()
    }
}

#[derive(Debug, Serialize)]
struct ApiMessage<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<Cow<'static, str>>,
}

impl<T: Serialize> Default for ApiMessage<T> {
    fn default() -> Self {
        Self {
            data: None,
            message: None,
        }
    }
}
