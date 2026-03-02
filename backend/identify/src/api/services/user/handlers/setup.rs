use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use identify_application::{
    AddUserToGroupParams, CreateGroupParams, CreateUserParams,
    GetSystemSettingsParams, GroupUseCaseDeps, SystemSettingsUseCaseDeps,
    UpsertSystemSettingsParams, UserUseCaseDeps, add_user_to_group,
    create_group, create_user, get_system_settings, update_system_settings,
};
use identify_domain::{NewGroupAttrs, NewUserAttrs};
use identify_infrastructure::{
    password::hash_password,
    storage::{
        ToShared as _, groups::GroupsRepository,
        system_settings::SystemSettingsRepository, users::UsersRepository,
    },
};
use tracing::trace;

use crate::api::services::{
    ApiResult,
    error::ApiError,
    response::ApiResponse,
    user::{
        UserServiceState,
        models::{requests::SetupRequest, responses::JwtTokenResponse},
    },
};

// FIXME: this handler should be moved to its own service -- the system service?

/// Performs a one-time system setup by creating an admin group and a user within it.
///
/// Returns an error if this was already done before.
pub async fn setup(
    State(state): State<UserServiceState>,
    Json(request): Json<SetupRequest>,
) -> ApiResult<JwtTokenResponse> {
    let tx = state.pool().begin().await?.to_shared();
    let now = Utc::now();

    // Do this in a separate scope to ensure that all references to the shared transaction are
    // dropped by the time we try to commit the transaction.
    let jwt_token = {
        let system_settings_repo = SystemSettingsRepository::new(tx.clone());

        // Check if this instance was already initialized.
        let mut settings = get_system_settings(
            SystemSettingsUseCaseDeps {
                repository: &system_settings_repo,
            },
            GetSystemSettingsParams,
        )
        .await
        .map_err(|e| {
            // This shouldn't ever fail, so it's fine to treat it as an internal error.
            ApiError::internal(Some(e))
        })?;

        if settings.is_initialized() {
            return Err(ApiError::new(
                "System is already initialized",
                StatusCode::CONFLICT,
                None,
            ));
        }

        let users_repo = UsersRepository::new(tx.clone());
        let groups_repo = GroupsRepository::new(tx.clone());

        // Create a new admin group.
        let group = create_group(
            GroupUseCaseDeps {
                repository: &groups_repo,
            },
            CreateGroupParams {
                group_attrs: NewGroupAttrs {
                    name: request.admin_group_name,
                    is_privileged: true,
                    created_at: now,
                },
            },
        )
        .await?;

        // Hash the password.
        let password_hash = tokio::task::spawn_blocking(move || {
            hash_password(&request.password)
        })
        .await
        .map_err(|err| ApiError::internal(Some(err)))??;

        // Create a new user.
        let user = create_user(
            UserUseCaseDeps {
                repository: &users_repo,
            },
            CreateUserParams {
                user_attrs: NewUserAttrs {
                    email: request.email,
                    first_name: request.first_name,
                    last_name: request.last_name,
                    password_hash: Some(password_hash),
                    created_at: now,
                },
            },
        )
        .await?;

        // Make created user an admin.
        add_user_to_group(
            GroupUseCaseDeps {
                repository: &groups_repo,
            },
            AddUserToGroupParams {
                user_id: user.id().to_uuid(),
                group_id: group.id().to_uuid(),
            },
        )
        .await?;

        // Mark the instance as initialized.
        settings.update_is_initialized(true);

        // Update system settings.
        update_system_settings(
            SystemSettingsUseCaseDeps {
                repository: &system_settings_repo,
            },
            UpsertSystemSettingsParams {
                settings: &settings,
            },
        )
        .await?;

        trace!("System was initialized!");

        state.jwt_client().issue_for_user(&user)?
    };

    // Commit the changes.
    tx.into_inner()?.commit().await?;

    // Return the newly generated token.
    Ok(ApiResponse::Data(JwtTokenResponse::new(jwt_token)))
}
