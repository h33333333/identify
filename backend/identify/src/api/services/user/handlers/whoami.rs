use axum::Extension;

use crate::api::{
    middleware::extensions::UserInformationExtension,
    services::{
        ApiResult, response::ApiResponse,
        user::models::responses::ViewUserResponse,
    },
};

/// Returns details about the currently authenticated user.
pub async fn whoami(
    Extension(UserInformationExtension(user_info)): Extension<
        UserInformationExtension,
    >,
) -> ApiResult<ViewUserResponse> {
    Ok(ApiResponse::Data(ViewUserResponse::new(
        user_info.user.clone(),
        user_info.groups.clone(),
    )))
}
