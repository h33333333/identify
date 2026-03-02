use axum::extract::{Path, State};
use identify_application::{
    GetUserParams, GroupUseCaseDeps, ListGroupsFilters, ListGroupsParams,
    UserUseCaseDeps, get_user, list_groups,
};
use identify_infrastructure::storage::{
    ToShared as _, groups::GroupsRepository, users::UsersRepository,
};
use uuid::Uuid;

use crate::api::services::{
    ApiResult,
    response::ApiResponse,
    user::{UserServiceState, models::responses::ViewUserResponse},
};

/// Returns details about a specific user.
pub async fn view(
    State(state): State<UserServiceState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<ViewUserResponse> {
    let tx = state.pool().begin().await?.to_shared();

    let users_repo = UsersRepository::new(tx.clone());

    let user = get_user(
        UserUseCaseDeps {
            repository: &users_repo,
        },
        GetUserParams { id: user_id },
    )
    .await?;

    let groups_repo = GroupsRepository::new(tx);
    let groups = list_groups(
        GroupUseCaseDeps {
            repository: &groups_repo,
        },
        ListGroupsParams {
            filters: ListGroupsFilters {
                user_id: Some(user_id),
            },
        },
    )
    .await?;

    Ok(ApiResponse::Data(ViewUserResponse::new(user, groups)))
}
