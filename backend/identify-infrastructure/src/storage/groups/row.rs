use chrono::{DateTime, Utc};
use identify_domain::{DomainError, Group, GroupAttrs};
use uuid::Uuid;

pub struct GroupRow {
    pub id: Uuid,
    pub name: String,
    pub is_privileged: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&Group> for GroupRow {
    fn from(value: &Group) -> Self {
        let attrs = value.to_attributes();

        GroupRow {
            id: attrs.id,
            name: attrs.name,
            is_privileged: attrs.is_privileged,
            created_at: attrs.created_at,
            updated_at: attrs.updated_at,
        }
    }
}

impl TryFrom<GroupRow> for Group {
    type Error = DomainError;

    fn try_from(value: GroupRow) -> Result<Self, Self::Error> {
        Group::load(GroupAttrs {
            id: value.id,
            name: value.name,
            is_privileged: value.is_privileged,
            created_at: value.created_at,
            updated_at: value.updated_at,
        })
    }
}
