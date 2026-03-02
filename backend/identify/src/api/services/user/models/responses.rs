use chrono::{DateTime, Utc};
use identify_domain::{Group, User};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ViewUserResponse {
    user: UserResponse,
    is_privileged: bool,
    groups: Vec<GroupResponse>,
}

impl ViewUserResponse {
    pub fn new(user: User, groups: Vec<Group>) -> Self {
        let user_response = user.into();

        let is_privileged = groups.iter().any(|group| group.is_privileged());

        let groups_response =
            groups.into_iter().map(Into::into).collect::<Vec<_>>();

        ViewUserResponse {
            user: user_response,
            is_privileged,
            groups: groups_response,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    id: Uuid,
    first_name: String,
    last_name: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        let attrs = value.to_attributes();
        UserResponse {
            id: attrs.id,
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GroupResponse {
    id: Uuid,
    is_privileged: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Group> for GroupResponse {
    fn from(value: Group) -> Self {
        GroupResponse {
            id: value.id().to_uuid(),
            is_privileged: value.is_privileged(),
            created_at: value.created_at(),
            updated_at: value.updated_at(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct JwtTokenResponse {
    token: String,
}

impl JwtTokenResponse {
    pub fn new(token: impl Into<String>) -> Self {
        JwtTokenResponse {
            token: token.into(),
        }
    }
}
