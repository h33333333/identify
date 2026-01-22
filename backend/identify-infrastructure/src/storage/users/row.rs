use chrono::{DateTime, Utc};
use identify_domain::{DomainError, User, UserAttrs};
use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&User> for UserRow {
    fn from(value: &User) -> Self {
        let attrs = value.to_attributes();

        UserRow {
            id: attrs.id,
            email: attrs.email,
            first_name: attrs.first_name,
            last_name: attrs.last_name,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        }
    }
}

impl TryFrom<UserRow> for User {
    type Error = DomainError;

    fn try_from(value: UserRow) -> Result<Self, Self::Error> {
        User::load(UserAttrs {
            id: value.id,
            email: value.email,
            first_name: value.first_name,
            last_name: value.last_name,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
