use identify_macros::gen_id;
use uuid::Uuid;

use crate::entities::UUID_NAMESPACE;
use crate::{DomainError, Result};

gen_id! {
    UUID_NAMESPACE,
    /// A stable and deterministic ID that uniquely identifies a [User](super::User) within the system.
    #[derive(Debug, Clone)]
    pub struct UserId {
        /// Email of the user.
        email: String,
    }

    #[derive(Debug)]
    pub struct UserIdAttrs;
}

impl UserId {
    pub fn new(attrs: UserIdAttrs) -> Self {
        UserId { email: attrs.email }
    }

    pub fn load(attrs: UserIdAttrs, expected: Uuid) -> Result<Self> {
        let id = UserId { email: attrs.email };

        let generated = id.to_uuid();

        if generated != expected {
            return Err(DomainError::id_mismatch(
                "UserId",
                format!("expected {}, got {}", expected, generated),
            ));
        }

        Ok(id)
    }
}
